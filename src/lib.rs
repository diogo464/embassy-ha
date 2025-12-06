#![no_std]

use core::{cell::RefCell, task::Waker};

use embassy_sync::waitqueue::AtomicWaker;
use heapless::{
    Vec, VecView,
    string::{String, StringView},
};
use serde::Serialize;

pub mod log;
pub use log::Format;

pub mod constants;

mod binary_state;
pub use binary_state::*;

mod entity;
pub use entity::*;

mod entity_binary_sensor;
pub use entity_binary_sensor::*;

mod entity_button;
pub use entity_button::*;

mod entity_category;
pub use entity_category::*;

mod entity_number;
pub use entity_number::*;

mod entity_sensor;
pub use entity_sensor::*;

mod entity_switch;
pub use entity_switch::*;

mod transport;
pub use transport::Transport;

mod unit;
pub use unit::*;

const AVAILABLE_PAYLOAD: &str = "online";
const NOT_AVAILABLE_PAYLOAD: &str = "offline";

#[derive(Debug)]
pub struct Error(&'static str);

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0)
    }
}

impl core::error::Error for Error {}

impl Error {
    pub(crate) fn new(message: &'static str) -> Self {
        Self(message)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "defmt", derive(Format))]
struct DeviceDiscovery<'a> {
    identifiers: &'a [&'a str],
    name: &'a str,
    manufacturer: &'a str,
    model: &'a str,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "defmt", derive(Format))]
struct EntityDiscovery<'a> {
    #[serde(rename = "unique_id")]
    id: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_class: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    state_topic: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    command_topic: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    unit_of_measurement: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    state_class: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    entity_picture: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_display_precision: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    availability_topic: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    payload_available: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    payload_not_available: Option<&'a str>,

    device: &'a DeviceDiscovery<'a>,
}

struct DiscoveryTopicDisplay<'a> {
    domain: &'a str,
    device_id: &'a str,
    entity_id: &'a str,
}

impl<'a> core::fmt::Display for DiscoveryTopicDisplay<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "homeassistant/{}/{}_{}/config",
            self.domain, self.device_id, self.entity_id
        )
    }
}

struct StateTopicDisplay<'a> {
    device_id: &'a str,
    entity_id: &'a str,
}

impl<'a> core::fmt::Display for StateTopicDisplay<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "embassy-ha/{}/{}/state", self.device_id, self.entity_id)
    }
}

struct CommandTopicDisplay<'a> {
    device_id: &'a str,
    entity_id: &'a str,
}

impl<'a> core::fmt::Display for CommandTopicDisplay<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "embassy-ha/{}/{}/command",
            self.device_id, self.entity_id
        )
    }
}

struct DeviceAvailabilityTopic<'a> {
    device_id: &'a str,
}

impl<'a> core::fmt::Display for DeviceAvailabilityTopic<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "embassy-ha/{}/availability", self.device_id)
    }
}

pub struct DeviceConfig {
    pub device_id: &'static str,
    pub device_name: &'static str,
    pub manufacturer: &'static str,
    pub model: &'static str,
}

pub struct DeviceResources {
    waker: AtomicWaker,
    entities: [RefCell<Option<EntityData>>; Self::ENTITY_LIMIT],

    mqtt_resources: embedded_mqtt::ClientResources,
    publish_buffer: Vec<u8, 2048>,
    subscribe_buffer: Vec<u8, 128>,
    discovery_buffer: Vec<u8, 2048>,
    availability_topic_buffer: String<128>,
    discovery_topic_buffer: String<128>,
    state_topic_buffer: String<128>,
    command_topic_buffer: String<128>,
}

impl DeviceResources {
    const ENTITY_LIMIT: usize = 16;
}

impl Default for DeviceResources {
    fn default() -> Self {
        Self {
            waker: AtomicWaker::new(),
            entities: [const { RefCell::new(None) }; Self::ENTITY_LIMIT],

            mqtt_resources: Default::default(),
            publish_buffer: Default::default(),
            subscribe_buffer: Default::default(),
            discovery_buffer: Default::default(),
            availability_topic_buffer: Default::default(),
            discovery_topic_buffer: Default::default(),
            state_topic_buffer: Default::default(),
            command_topic_buffer: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ButtonStorage {
    pub timestamp: Option<embassy_time::Instant>,
    pub consumed: bool,
}

#[derive(Debug)]
pub struct SwitchCommand {
    pub value: BinaryState,
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug)]
pub struct SwitchState {
    pub value: BinaryState,
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug, Default)]
pub struct SwitchStorage {
    pub state: Option<SwitchState>,
    pub command: Option<SwitchCommand>,
    pub publish_on_command: bool,
}

#[derive(Debug)]
pub struct BinarySensorState {
    pub value: BinaryState,
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug, Default)]
pub struct BinarySensorStorage {
    pub state: Option<BinarySensorState>,
}

#[derive(Debug)]
pub struct NumericSensorState {
    pub value: f32,
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug, Default)]
pub struct NumericSensorStorage {
    pub state: Option<NumericSensorState>,
}

#[derive(Debug)]
pub struct NumberState {
    pub value: f32,
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug)]
pub struct NumberCommand {
    pub value: f32,
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug, Default)]
pub struct NumberStorage {
    pub state: Option<NumberState>,
    pub command: Option<NumberCommand>,
    pub publish_on_command: bool,
}

#[derive(Debug)]
pub enum EntityStorage {
    Button(ButtonStorage),
    Switch(SwitchStorage),
    BinarySensor(BinarySensorStorage),
    NumericSensor(NumericSensorStorage),
    Number(NumberStorage),
}

impl EntityStorage {
    pub fn as_button_mut(&mut self) -> &mut ButtonStorage {
        match self {
            EntityStorage::Button(storage) => storage,
            _ => panic!("expected storage type to be button"),
        }
    }

    pub fn as_switch_mut(&mut self) -> &mut SwitchStorage {
        match self {
            EntityStorage::Switch(storage) => storage,
            _ => panic!("expected storage type to be switch"),
        }
    }

    pub fn as_binary_sensor_mut(&mut self) -> &mut BinarySensorStorage {
        match self {
            EntityStorage::BinarySensor(storage) => storage,
            _ => panic!("expected storage type to be binary_sensor"),
        }
    }

    pub fn as_numeric_sensor_mut(&mut self) -> &mut NumericSensorStorage {
        match self {
            EntityStorage::NumericSensor(storage) => storage,
            _ => panic!("expected storage type to be numeric_sensor"),
        }
    }

    pub fn as_number_mut(&mut self) -> &mut NumberStorage {
        match self {
            EntityStorage::Number(storage) => storage,
            _ => panic!("expected storage type to be number"),
        }
    }
}

struct EntityData {
    config: EntityConfig,
    storage: EntityStorage,
    publish: bool,
    command: bool,
    command_waker: Option<Waker>,
}

pub struct Entity<'a> {
    pub(crate) data: &'a RefCell<Option<EntityData>>,
    pub(crate) waker: &'a AtomicWaker,
}

impl<'a> Entity<'a> {
    pub fn queue_publish(&mut self) {
        self.with_data(|data| data.publish = true);
        self.waker.wake();
    }

    pub async fn wait_command(&mut self) {
        struct Fut<'a, 'b>(&'a mut Entity<'b>);

        impl<'a, 'b> core::future::Future for Fut<'a, 'b> {
            type Output = ();

            fn poll(
                mut self: core::pin::Pin<&mut Self>,
                cx: &mut core::task::Context<'_>,
            ) -> core::task::Poll<Self::Output> {
                let this = &mut self.as_mut().0;
                this.with_data(|data| {
                    let dirty = data.command;
                    if dirty {
                        data.command = false;
                        data.command_waker = None;
                        core::task::Poll::Ready(())
                    } else {
                        // TODO: avoid clone if waker would wake
                        data.command_waker = Some(cx.waker().clone());
                        core::task::Poll::Pending
                    }
                })
            }
        }

        Fut(self).await
    }

    fn with_data<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut EntityData) -> R,
    {
        f(self.data.borrow_mut().as_mut().unwrap())
    }
}

pub struct Device<'a> {
    config: DeviceConfig,

    // resources
    waker: &'a AtomicWaker,
    entities: &'a [RefCell<Option<EntityData>>],

    mqtt_resources: &'a mut embedded_mqtt::ClientResources,
    publish_buffer: &'a mut VecView<u8>,
    subscribe_buffer: &'a mut VecView<u8>,
    discovery_buffer: &'a mut VecView<u8>,
    availability_topic_buffer: &'a mut StringView,
    discovery_topic_buffer: &'a mut StringView,
    state_topic_buffer: &'a mut StringView,
    command_topic_buffer: &'a mut StringView,
}

impl<'a> Device<'a> {
    pub fn new(resources: &'a mut DeviceResources, config: DeviceConfig) -> Self {
        Self {
            config,
            waker: &resources.waker,
            entities: &resources.entities,

            mqtt_resources: &mut resources.mqtt_resources,
            publish_buffer: &mut resources.publish_buffer,
            subscribe_buffer: &mut resources.subscribe_buffer,
            discovery_buffer: &mut resources.discovery_buffer,
            availability_topic_buffer: &mut resources.availability_topic_buffer,
            discovery_topic_buffer: &mut resources.discovery_topic_buffer,
            state_topic_buffer: &mut resources.state_topic_buffer,
            command_topic_buffer: &mut resources.command_topic_buffer,
        }
    }

    pub fn create_entity(&self, config: EntityConfig, storage: EntityStorage) -> Entity<'a> {
        let index = 'outer: {
            for idx in 0..self.entities.len() {
                if self.entities[idx].borrow().is_none() {
                    break 'outer idx;
                }
            }
            panic!("device entity limit reached");
        };

        let data = EntityData {
            config,
            storage,
            publish: false,
            command: false,
            command_waker: None,
        };
        self.entities[index].replace(Some(data));

        Entity {
            data: &self.entities[index],
            waker: self.waker,
        }
    }

    pub fn create_sensor(&self, id: &'static str, config: SensorConfig) -> Sensor<'a> {
        let mut entity_config = EntityConfig::default();
        entity_config.id = id;
        config.populate(&mut entity_config);

        let entity = self.create_entity(
            entity_config,
            EntityStorage::NumericSensor(Default::default()),
        );
        Sensor::new(entity)
    }

    pub fn create_button(&self, id: &'static str, config: ButtonConfig) -> Button<'a> {
        let mut entity_config = EntityConfig::default();
        entity_config.id = id;
        config.populate(&mut entity_config);

        let entity = self.create_entity(entity_config, EntityStorage::Button(Default::default()));
        Button::new(entity)
    }

    pub fn create_number(&self, id: &'static str, config: NumberConfig) -> Number<'a> {
        let mut entity_config = EntityConfig::default();
        entity_config.id = id;
        config.populate(&mut entity_config);

        let entity = self.create_entity(
            entity_config,
            EntityStorage::Number(NumberStorage {
                publish_on_command: config.publish_on_command,
                ..Default::default()
            }),
        );
        Number::new(entity)
    }

    pub fn create_switch(&self, id: &'static str, config: SwitchConfig) -> Switch<'a> {
        let mut entity_config = EntityConfig::default();
        entity_config.id = id;
        config.populate(&mut entity_config);

        let entity = self.create_entity(
            entity_config,
            EntityStorage::Switch(SwitchStorage {
                publish_on_command: config.publish_on_command,
                ..Default::default()
            }),
        );
        Switch::new(entity)
    }

    pub fn create_binary_sensor(
        &self,
        id: &'static str,
        config: BinarySensorConfig,
    ) -> BinarySensor<'a> {
        let mut entity_config = EntityConfig::default();
        entity_config.id = id;
        config.populate(&mut entity_config);

        let entity = self.create_entity(
            entity_config,
            EntityStorage::BinarySensor(Default::default()),
        );
        BinarySensor::new(entity)
    }

    pub async fn run<T: Transport>(&mut self, transport: &mut T) -> Result<(), Error> {
        use core::fmt::Write;

        self.availability_topic_buffer.clear();
        write!(
            self.availability_topic_buffer,
            "{}",
            DeviceAvailabilityTopic {
                device_id: self.config.device_id
            }
        )
        .expect("device availability buffer too small");
        let availability_topic = self.availability_topic_buffer.as_str();

        let mut client = embedded_mqtt::Client::new(self.mqtt_resources, transport);
        let connect_params = embedded_mqtt::ConnectParams {
            will_topic: Some(availability_topic),
            will_payload: Some(NOT_AVAILABLE_PAYLOAD.as_bytes()),
            ..Default::default()
        };
        if let Err(err) = client
            .connect_with(self.config.device_id, connect_params)
            .await
        {
            crate::log::error!(
                "mqtt connect failed with: {:?}",
                crate::log::Debug2Format(&err)
            );
            return Err(Error::new("mqtt connection failed"));
        }

        crate::log::debug!("sending discover messages");
        let device_discovery = DeviceDiscovery {
            identifiers: &[self.config.device_id],
            name: self.config.device_name,
            manufacturer: self.config.manufacturer,
            model: self.config.model,
        };

        for entity in self.entities {
            self.publish_buffer.clear();
            self.subscribe_buffer.clear();
            self.discovery_buffer.clear();
            self.discovery_topic_buffer.clear();
            self.state_topic_buffer.clear();
            self.command_topic_buffer.clear();

            // borrow the entity and fill out the buffers to be sent
            // this should be done inside a block so that we do not hold the RefMut across an
            // await
            {
                let mut entity = entity.borrow_mut();
                let entity = match entity.as_mut() {
                    Some(entity) => entity,
                    None => break,
                };
                let entity_config = &entity.config;

                let discovery_topic_display = DiscoveryTopicDisplay {
                    domain: entity_config.domain,
                    device_id: self.config.device_id,
                    entity_id: entity_config.id,
                };
                let state_topic_display = StateTopicDisplay {
                    device_id: self.config.device_id,
                    entity_id: entity_config.id,
                };
                let command_topic_display = CommandTopicDisplay {
                    device_id: self.config.device_id,
                    entity_id: entity_config.id,
                };

                write!(self.discovery_topic_buffer, "{discovery_topic_display}")
                    .expect("discovery topic buffer too small");
                write!(self.state_topic_buffer, "{state_topic_display}")
                    .expect("state topic buffer too small");
                write!(self.command_topic_buffer, "{command_topic_display}")
                    .expect("command topic buffer too small");

                let discovery = EntityDiscovery {
                    id: entity_config.id,
                    name: entity_config.name,
                    device_class: entity_config.device_class,
                    state_topic: Some(self.state_topic_buffer.as_str()),
                    command_topic: Some(self.command_topic_buffer.as_str()),
                    unit_of_measurement: entity_config.measurement_unit,
                    schema: entity_config.schema,
                    state_class: entity_config.state_class,
                    icon: entity_config.icon,
                    entity_picture: entity_config.picture,
                    min: entity_config.min,
                    max: entity_config.max,
                    step: entity_config.step,
                    mode: entity_config.mode,
                    suggested_display_precision: entity_config.suggested_display_precision,
                    availability_topic: Some(availability_topic),
                    payload_available: Some(AVAILABLE_PAYLOAD),
                    payload_not_available: Some(NOT_AVAILABLE_PAYLOAD),
                    device: &device_discovery,
                };
                crate::log::debug!(
                    "discovery for entity '{}': {:?}",
                    entity_config.id,
                    discovery
                );

                self.discovery_buffer
                    .resize(self.discovery_buffer.capacity(), 0)
                    .unwrap();
                let n = serde_json_core::to_slice(&discovery, &mut self.discovery_buffer)
                    .expect("discovery buffer too small");
                self.discovery_buffer.truncate(n);
            }

            let discovery_topic = self.discovery_topic_buffer.as_str();
            crate::log::debug!("sending discovery to topic '{}'", discovery_topic);
            if let Err(err) = client
                .publish(discovery_topic, &self.discovery_buffer)
                .await
            {
                crate::log::error!(
                    "mqtt discovery publish failed with: {:?}",
                    crate::log::Debug2Format(&err)
                );
                return Err(Error::new("mqtt discovery publish failed"));
            }

            let command_topic = self.command_topic_buffer.as_str();
            crate::log::debug!("subscribing to command topic '{}'", command_topic);
            if let Err(err) = client.subscribe(command_topic).await {
                crate::log::error!(
                    "mqtt subscribe to '{}' failed with: {:?}",
                    command_topic,
                    crate::log::Debug2Format(&err)
                );
                return Err(Error::new(
                    "mqtt subscription to entity command topic failed",
                ));
            }
        }

        if let Err(err) = client
            .publish(availability_topic, AVAILABLE_PAYLOAD.as_bytes())
            .await
        {
            crate::log::error!(
                "mqtt availability publish failed with: {:?}",
                crate::log::Debug2Format(&err)
            );
            return Err(Error::new("mqtt availability publish failed"));
        }

        'outer_loop: loop {
            use core::fmt::Write;

            for entity in self.entities {
                {
                    let mut entity = entity.borrow_mut();
                    let entity = match entity.as_mut() {
                        Some(entity) => entity,
                        None => break,
                    };

                    if !entity.publish {
                        continue;
                    }

                    entity.publish = false;
                    self.publish_buffer.clear();

                    match &entity.storage {
                        EntityStorage::Switch(SwitchStorage {
                            state: Some(SwitchState { value, .. }),
                            ..
                        }) => self
                            .publish_buffer
                            .extend_from_slice(value.as_str().as_bytes())
                            .expect("publish buffer too small for switch state payload"),
                        EntityStorage::BinarySensor(BinarySensorStorage {
                            state: Some(BinarySensorState { value, .. }),
                        }) => self
                            .publish_buffer
                            .extend_from_slice(value.as_str().as_bytes())
                            .expect("publish buffer too small for binary sensor state payload"),
                        EntityStorage::NumericSensor(NumericSensorStorage {
                            state: Some(NumericSensorState { value, .. }),
                            ..
                        }) => write!(self.publish_buffer, "{}", value)
                            .expect("publish buffer too small for numeric sensor payload"),
                        EntityStorage::Number(NumberStorage {
                            state: Some(NumberState { value, .. }),
                            ..
                        }) => write!(self.publish_buffer, "{}", value)
                            .expect("publish buffer too small for number state payload"),
                        _ => {
                            crate::log::warn!(
                                "entity '{}' requested state publish but its storage does not support it",
                                entity.config.id
                            );
                            continue;
                        }
                    }

                    let state_topic_display = StateTopicDisplay {
                        device_id: self.config.device_id,
                        entity_id: entity.config.id,
                    };
                    self.state_topic_buffer.clear();
                    write!(self.state_topic_buffer, "{state_topic_display}")
                        .expect("state topic buffer too small");
                }

                let state_topic = self.state_topic_buffer.as_str();
                if let Err(err) = client.publish(state_topic, self.publish_buffer).await {
                    crate::log::error!(
                        "mqtt state publish on topic '{}' failed with: {:?}",
                        state_topic,
                        crate::log::Debug2Format(&err)
                    );
                    return Err(Error::new("mqtt publish failed"));
                }
            }

            let receive = client.receive();
            let waker = wait_on_atomic_waker(self.waker);
            let publish = match embassy_futures::select::select(receive, waker).await {
                embassy_futures::select::Either::First(packet) => match packet {
                    Ok(embedded_mqtt::Packet::Publish(publish)) => publish,
                    Err(err) => {
                        crate::log::error!(
                            "mqtt receive failed with: {:?}",
                            crate::log::Debug2Format(&err)
                        );
                        return Err(Error::new("mqtt receive failed"));
                    }
                    _ => continue,
                },
                embassy_futures::select::Either::Second(_) => continue,
            };

            let entity = 'entity_search_block: {
                for entity in self.entities {
                    let mut data = entity.borrow_mut();
                    let data = match data.as_mut() {
                        Some(data) => data,
                        None => break,
                    };

                    let command_topic_display = CommandTopicDisplay {
                        device_id: self.config.device_id,
                        entity_id: data.config.id,
                    };
                    self.command_topic_buffer.clear();
                    write!(self.command_topic_buffer, "{command_topic_display}")
                        .expect("command topic buffer too small");

                    if self.command_topic_buffer.as_bytes() == publish.topic.as_bytes() {
                        break 'entity_search_block entity;
                    }
                }
                continue 'outer_loop;
            };

            let mut read_buffer = [0u8; 128];
            if publish.data_len > read_buffer.len() {
                crate::log::warn!(
                    "mqtt publish payload on topic {} is too large ({} bytes), ignoring it",
                    publish.topic,
                    publish.data_len
                );
                continue;
            }

            crate::log::debug!(
                "mqtt receiving {} bytes of data on topic {}",
                publish.data_len,
                publish.topic
            );

            let data_len = publish.data_len;
            if let Err(err) = client.receive_data(&mut read_buffer[..data_len]).await {
                crate::log::error!(
                    "mqtt receive data failed with: {:?}",
                    crate::log::Debug2Format(&err)
                );
                return Err(Error::new("mqtt receive data failed"));
            }

            let command = match str::from_utf8(&read_buffer[..data_len]) {
                Ok(command) => command,
                Err(_) => {
                    crate::log::warn!("mqtt message contained invalid utf-8, ignoring it");
                    continue;
                }
            };

            let mut entity = entity.borrow_mut();
            let data = entity.as_mut().unwrap();

            match &mut data.storage {
                EntityStorage::Button(button_storage) => {
                    if command != constants::HA_BUTTON_PAYLOAD_PRESS {
                        crate::log::warn!(
                            "button '{}' received unexpected command '{}', expected '{}', ignoring it",
                            data.config.id,
                            command,
                            constants::HA_BUTTON_PAYLOAD_PRESS
                        );
                        continue;
                    }
                    button_storage.consumed = false;
                    button_storage.timestamp = Some(embassy_time::Instant::now());
                }
                EntityStorage::Switch(switch_storage) => {
                    let command = match command.parse::<BinaryState>() {
                        Ok(command) => command,
                        Err(_) => {
                            crate::log::warn!(
                                "switch '{}' received invalid command '{}', expected 'ON' or 'OFF', ignoring it",
                                data.config.id,
                                command
                            );
                            continue;
                        }
                    };
                    let timestamp = embassy_time::Instant::now();
                    if switch_storage.publish_on_command {
                        data.publish = true;
                        switch_storage.state = Some(SwitchState {
                            value: command,
                            timestamp,
                        });
                    }
                    switch_storage.command = Some(SwitchCommand {
                        value: command,
                        timestamp,
                    });
                }
                EntityStorage::Number(number_storage) => {
                    let command = match command.parse::<f32>() {
                        Ok(command) => command,
                        Err(_) => {
                            crate::log::warn!(
                                "number '{}' received invalid command '{}', expected a valid number, ignoring it",
                                data.config.id,
                                command
                            );
                            continue;
                        }
                    };
                    let timestamp = embassy_time::Instant::now();
                    if number_storage.publish_on_command {
                        data.publish = true;
                        number_storage.state = Some(NumberState {
                            value: command,
                            timestamp,
                        });
                    }
                    number_storage.command = Some(NumberCommand {
                        value: command,
                        timestamp,
                    });
                }
                _ => continue 'outer_loop,
            }

            data.command = true;
            if let Some(waker) = data.command_waker.take() {
                waker.wake();
            }
        }
    }
}

async fn wait_on_atomic_waker(waker: &AtomicWaker) {
    struct F<'a>(&'a AtomicWaker, bool);
    impl<'a> core::future::Future for F<'a> {
        type Output = ();

        fn poll(
            self: core::pin::Pin<&mut Self>,
            cx: &mut core::task::Context<'_>,
        ) -> core::task::Poll<Self::Output> {
            if !self.1 {
                self.0.register(cx.waker());
                self.get_mut().1 = true;
                core::task::Poll::Pending
            } else {
                core::task::Poll::Ready(())
            }
        }
    }
    F(waker, false).await
}
