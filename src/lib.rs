//! Home Assistant MQTT device library for embassy.
//!
//! To create a device use the [`new`] function.
//!
//! After the device is created you should create one or more entities using functions such as
//! [`create_button`]/[`create_sensor`]/...
//!
//! Once the entities have been created either [`run`] or [`connect_and_run`] should be called in a
//! seperate task.
//!
//! There are various examples you can run locally (ex: `cargo run --features tracing --example
//! button`) assuming you have a home assistant instance running. To run the examples the
//! environment variable `MQTT_ADDRESS` should be set to the mqtt server used by home assistant.

#![no_std]

use core::{
    cell::RefCell,
    net::{Ipv4Addr, SocketAddrV4},
    task::Waker,
};

use embassy_net::tcp::TcpSocket;
use embassy_sync::waitqueue::AtomicWaker;
use embassy_time::{Duration, Timer};
use heapless::{
    Vec, VecView,
    string::{String, StringView},
};
use serde::Serialize;

mod mqtt;

mod log;
#[allow(unused)]
use log::Format;

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

mod entity_device_tracker;
pub use entity_device_tracker::*;

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
const DEFAULT_KEEPALIVE_TIME: u16 = 30;

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
    json_attributes_topic: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    unit_of_measurement: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    state_class: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    entity_category: Option<&'a str>,

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

struct AttributesTopicDisplay<'a> {
    device_id: &'a str,
    entity_id: &'a str,
}

impl<'a> core::fmt::Display for AttributesTopicDisplay<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "embassy-ha/{}/{}/attributes",
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

    mqtt_resources: mqtt::ClientResources,
    publish_buffer: Vec<u8, 2048>,
    subscribe_buffer: Vec<u8, 128>,
    discovery_buffer: Vec<u8, 2048>,
    availability_topic_buffer: String<128>,
    discovery_topic_buffer: String<128>,
    state_topic_buffer: String<128>,
    command_topic_buffer: String<128>,
    attributes_topic_buffer: String<128>,
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
            attributes_topic_buffer: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct ButtonStorage {
    pub timestamp: Option<embassy_time::Instant>,
    pub consumed: bool,
}

#[derive(Debug)]
pub(crate) struct SwitchCommand {
    pub value: BinaryState,
    #[allow(unused)]
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug)]
pub(crate) struct SwitchState {
    pub value: BinaryState,
    #[allow(unused)]
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug, Default)]
pub(crate) struct SwitchStorage {
    pub state: Option<SwitchState>,
    pub command: Option<SwitchCommand>,
    pub publish_on_command: bool,
}

#[derive(Debug)]
pub(crate) struct BinarySensorState {
    pub value: BinaryState,
    #[allow(unused)]
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug, Default)]
pub(crate) struct BinarySensorStorage {
    pub state: Option<BinarySensorState>,
}

#[derive(Debug)]
pub(crate) struct NumericSensorState {
    pub value: f32,
    #[allow(unused)]
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug, Default)]
pub(crate) struct NumericSensorStorage {
    pub state: Option<NumericSensorState>,
}

#[derive(Debug)]
pub(crate) struct NumberState {
    pub value: f32,
    #[allow(unused)]
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug)]
pub(crate) struct NumberCommand {
    pub value: f32,
    #[allow(unused)]
    pub timestamp: embassy_time::Instant,
}

#[derive(Debug, Default)]
pub(crate) struct NumberStorage {
    pub state: Option<NumberState>,
    pub command: Option<NumberCommand>,
    pub publish_on_command: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct DeviceTrackerState {
    pub latitude: f32,
    pub longitude: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_accuracy: Option<f32>,
}

#[derive(Debug, Default)]
pub(crate) struct DeviceTrackerStorage {
    pub state: Option<DeviceTrackerState>,
}

#[derive(Debug)]
pub(crate) enum EntityStorage {
    Button(ButtonStorage),
    Switch(SwitchStorage),
    BinarySensor(BinarySensorStorage),
    NumericSensor(NumericSensorStorage),
    Number(NumberStorage),
    DeviceTracker(DeviceTrackerStorage),
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

    pub fn as_device_tracker_mut(&mut self) -> &mut DeviceTrackerStorage {
        match self {
            EntityStorage::DeviceTracker(storage) => storage,
            _ => panic!("expected storage type to be device tracker"),
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

pub(crate) struct Entity<'a> {
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

    mqtt_resources: &'a mut mqtt::ClientResources,
    publish_buffer: &'a mut VecView<u8>,
    subscribe_buffer: &'a mut VecView<u8>,
    discovery_buffer: &'a mut VecView<u8>,
    availability_topic_buffer: &'a mut StringView,
    discovery_topic_buffer: &'a mut StringView,
    state_topic_buffer: &'a mut StringView,
    command_topic_buffer: &'a mut StringView,
    attributes_topic_buffer: &'a mut StringView,
}

pub fn new<'a>(resources: &'a mut DeviceResources, config: DeviceConfig) -> Device<'a> {
    Device {
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
        attributes_topic_buffer: &mut resources.attributes_topic_buffer,
    }
}

fn create_entity<'a>(
    device: &Device<'a>,
    config: EntityConfig,
    storage: EntityStorage,
) -> Entity<'a> {
    let index = 'outer: {
        for idx in 0..device.entities.len() {
            if device.entities[idx].borrow().is_none() {
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
    device.entities[index].replace(Some(data));

    Entity {
        data: &device.entities[index],
        waker: device.waker,
    }
}

pub fn create_sensor<'a>(
    device: &Device<'a>,
    id: &'static str,
    config: SensorConfig,
) -> Sensor<'a> {
    let mut entity_config = EntityConfig {
        id,
        ..Default::default()
    };
    config.populate(&mut entity_config);

    let entity = create_entity(
        device,
        entity_config,
        EntityStorage::NumericSensor(Default::default()),
    );
    Sensor::new(entity)
}

pub fn create_button<'a>(
    device: &Device<'a>,
    id: &'static str,
    config: ButtonConfig,
) -> Button<'a> {
    let mut entity_config = EntityConfig {
        id,
        ..Default::default()
    };
    config.populate(&mut entity_config);

    let entity = create_entity(
        device,
        entity_config,
        EntityStorage::Button(Default::default()),
    );
    Button::new(entity)
}

pub fn create_number<'a>(
    device: &Device<'a>,
    id: &'static str,
    config: NumberConfig,
) -> Number<'a> {
    let mut entity_config = EntityConfig {
        id,
        ..Default::default()
    };
    config.populate(&mut entity_config);

    let entity = create_entity(
        device,
        entity_config,
        EntityStorage::Number(NumberStorage {
            publish_on_command: config.publish_on_command,
            ..Default::default()
        }),
    );
    Number::new(entity)
}

pub fn create_switch<'a>(
    device: &Device<'a>,
    id: &'static str,
    config: SwitchConfig,
) -> Switch<'a> {
    let mut entity_config = EntityConfig {
        id,
        ..Default::default()
    };
    config.populate(&mut entity_config);

    let entity = create_entity(
        device,
        entity_config,
        EntityStorage::Switch(SwitchStorage {
            publish_on_command: config.publish_on_command,
            ..Default::default()
        }),
    );
    Switch::new(entity)
}

pub fn create_binary_sensor<'a>(
    device: &Device<'a>,
    id: &'static str,
    config: BinarySensorConfig,
) -> BinarySensor<'a> {
    let mut entity_config = EntityConfig {
        id,
        ..Default::default()
    };
    config.populate(&mut entity_config);

    let entity = create_entity(
        device,
        entity_config,
        EntityStorage::BinarySensor(Default::default()),
    );
    BinarySensor::new(entity)
}

pub fn create_device_tracker<'a>(
    device: &Device<'a>,
    id: &'static str,
    config: DeviceTrackerConfig,
) -> DeviceTracker<'a> {
    let mut entity_config = EntityConfig {
        id,
        ..Default::default()
    };
    config.populate(&mut entity_config);

    let entity = create_entity(
        device,
        entity_config,
        EntityStorage::DeviceTracker(Default::default()),
    );
    DeviceTracker::new(entity)
}

pub async fn run<T: Transport>(device: &mut Device<'_>, transport: &mut T) -> Result<(), Error> {
    use core::fmt::Write;

    const MQTT_TIMEOUT: Duration = Duration::from_secs(30);

    device.availability_topic_buffer.clear();
    write!(
        device.availability_topic_buffer,
        "{}",
        DeviceAvailabilityTopic {
            device_id: device.config.device_id
        }
    )
    .expect("device availability buffer too small");
    let availability_topic = device.availability_topic_buffer.as_str();

    let mut ping_ticker =
        embassy_time::Ticker::every(Duration::from_secs(u64::from(DEFAULT_KEEPALIVE_TIME)));
    let mut client = mqtt::Client::new(device.mqtt_resources, transport);
    let connect_params = mqtt::ConnectParams {
        will_topic: Some(availability_topic),
        will_payload: Some(NOT_AVAILABLE_PAYLOAD.as_bytes()),
        will_retain: true,
        keepalive: Some(DEFAULT_KEEPALIVE_TIME),
        ..Default::default()
    };
    match embassy_time::with_timeout(
        MQTT_TIMEOUT,
        client.connect_with(device.config.device_id, connect_params),
    )
    .await
    {
        Ok(Ok(())) => {}
        Ok(Err(err)) => {
            crate::log::error!(
                "mqtt connect failed with: {:?}",
                crate::log::Debug2Format(&err)
            );
            return Err(Error::new("mqtt connection failed"));
        }
        Err(_) => {
            crate::log::error!("mqtt connect timed out");
            return Err(Error::new("mqtt connect timed out"));
        }
    }

    crate::log::debug!("sending discover messages");
    let device_discovery = DeviceDiscovery {
        identifiers: &[device.config.device_id],
        name: device.config.device_name,
        manufacturer: device.config.manufacturer,
        model: device.config.model,
    };

    for entity in device.entities {
        device.publish_buffer.clear();
        device.subscribe_buffer.clear();
        device.discovery_buffer.clear();
        device.discovery_topic_buffer.clear();
        device.state_topic_buffer.clear();
        device.command_topic_buffer.clear();
        device.attributes_topic_buffer.clear();

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
                device_id: device.config.device_id,
                entity_id: entity_config.id,
            };
            let state_topic_display = StateTopicDisplay {
                device_id: device.config.device_id,
                entity_id: entity_config.id,
            };
            let command_topic_display = CommandTopicDisplay {
                device_id: device.config.device_id,
                entity_id: entity_config.id,
            };
            let attributes_topic_display = AttributesTopicDisplay {
                device_id: device.config.device_id,
                entity_id: entity_config.id,
            };

            write!(device.discovery_topic_buffer, "{discovery_topic_display}")
                .expect("discovery topic buffer too small");
            write!(device.state_topic_buffer, "{state_topic_display}")
                .expect("state topic buffer too small");
            write!(device.command_topic_buffer, "{command_topic_display}")
                .expect("command topic buffer too small");
            write!(device.attributes_topic_buffer, "{attributes_topic_display}")
                .expect("attributes topic buffer too small");

            let discovery = EntityDiscovery {
                id: entity_config.id,
                name: entity_config.name,
                device_class: entity_config.device_class,
                state_topic: Some(device.state_topic_buffer.as_str()),
                command_topic: Some(device.command_topic_buffer.as_str()),
                json_attributes_topic: Some(device.attributes_topic_buffer.as_str()),
                unit_of_measurement: entity_config.measurement_unit,
                schema: entity_config.schema,
                platform: entity_config.platform,
                state_class: entity_config.state_class,
                icon: entity_config.icon,
                entity_category: entity_config.category,
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

            device
                .discovery_buffer
                .resize(device.discovery_buffer.capacity(), 0)
                .unwrap();
            let n = serde_json_core::to_slice(&discovery, device.discovery_buffer)
                .expect("discovery buffer too small");
            device.discovery_buffer.truncate(n);
        }

        let discovery_topic = device.discovery_topic_buffer.as_str();
        crate::log::debug!("sending discovery to topic '{}'", discovery_topic);
        match embassy_time::with_timeout(
            MQTT_TIMEOUT,
            client.publish(discovery_topic, device.discovery_buffer),
        )
        .await
        {
            Ok(Ok(_)) => {}
            Ok(Err(err)) => {
                crate::log::error!(
                    "mqtt discovery publish failed with: {:?}",
                    crate::log::Debug2Format(&err)
                );
                return Err(Error::new("mqtt discovery publish failed"));
            }
            Err(_) => {
                crate::log::error!("mqtt discovery publish timed out");
                return Err(Error::new("mqtt discovery publish timed out"));
            }
        }

        let command_topic = device.command_topic_buffer.as_str();
        crate::log::debug!("subscribing to command topic '{}'", command_topic);
        match embassy_time::with_timeout(MQTT_TIMEOUT, client.subscribe(command_topic)).await {
            Ok(Ok(_)) => {}
            Ok(Err(err)) => {
                crate::log::error!(
                    "mqtt subscribe to '{}' failed with: {:?}",
                    command_topic,
                    crate::log::Debug2Format(&err)
                );
                return Err(Error::new(
                    "mqtt subscription to entity command topic failed",
                ));
            }
            Err(_) => {
                crate::log::error!("mqtt subscribe to '{}' timed out", command_topic);
                return Err(Error::new("mqtt subscribe timed out"));
            }
        }
    }

    match embassy_time::with_timeout(
        MQTT_TIMEOUT,
        client.publish_with(
            availability_topic,
            AVAILABLE_PAYLOAD.as_bytes(),
            mqtt::PublishParams {
                retain: true,
                ..Default::default()
            },
        ),
    )
    .await
    {
        Ok(Ok(_)) => {}
        Ok(Err(err)) => {
            crate::log::error!(
                "mqtt availability publish failed with: {:?}",
                crate::log::Debug2Format(&err)
            );
            return Err(Error::new("mqtt availability publish failed"));
        }
        Err(_) => {
            crate::log::error!("mqtt availability publish timed out");
            return Err(Error::new("mqtt availability publish timed out"));
        }
    }

    let mut first_iteration_push = true;
    'outer_loop: loop {
        use core::fmt::Write;

        for entity in device.entities {
            let publish_topic = {
                let mut entity = entity.borrow_mut();
                let entity = match entity.as_mut() {
                    Some(entity) => entity,
                    None => break,
                };

                if !entity.publish && !first_iteration_push {
                    continue;
                }

                entity.publish = false;
                device.publish_buffer.clear();

                let mut publish_to_attributes = false;
                match &entity.storage {
                    EntityStorage::Switch(SwitchStorage {
                        state: Some(SwitchState { value, .. }),
                        ..
                    }) => device
                        .publish_buffer
                        .extend_from_slice(value.as_str().as_bytes())
                        .expect("publish buffer too small for switch state payload"),
                    EntityStorage::BinarySensor(BinarySensorStorage {
                        state: Some(BinarySensorState { value, .. }),
                    }) => device
                        .publish_buffer
                        .extend_from_slice(value.as_str().as_bytes())
                        .expect("publish buffer too small for binary sensor state payload"),
                    EntityStorage::NumericSensor(NumericSensorStorage {
                        state: Some(NumericSensorState { value, .. }),
                        ..
                    }) => write!(device.publish_buffer, "{}", value)
                        .expect("publish buffer too small for numeric sensor payload"),
                    EntityStorage::Number(NumberStorage {
                        state: Some(NumberState { value, .. }),
                        ..
                    }) => write!(device.publish_buffer, "{}", value)
                        .expect("publish buffer too small for number state payload"),
                    EntityStorage::DeviceTracker(DeviceTrackerStorage {
                        state: Some(tracker_state),
                    }) => {
                        publish_to_attributes = true;
                        device
                            .publish_buffer
                            .resize(device.publish_buffer.capacity(), 0)
                            .expect("resize to capacity should never fail");
                        let n =
                            serde_json_core::to_slice(&tracker_state, &mut device.publish_buffer)
                                .expect("publish buffer too small for tracker state payload");
                        device.publish_buffer.truncate(n);
                    }
                    _ => {
                        if !first_iteration_push {
                            crate::log::warn!(
                                "entity '{}' requested state publish but its storage does not support it",
                                entity.config.id
                            );
                        }
                        continue;
                    }
                }

                if publish_to_attributes {
                    let attributes_topic_display = AttributesTopicDisplay {
                        device_id: device.config.device_id,
                        entity_id: entity.config.id,
                    };
                    device.attributes_topic_buffer.clear();
                    write!(device.attributes_topic_buffer, "{attributes_topic_display}")
                        .expect("attributes topic buffer too small");
                    device.attributes_topic_buffer.as_str()
                } else {
                    let state_topic_display = StateTopicDisplay {
                        device_id: device.config.device_id,
                        entity_id: entity.config.id,
                    };
                    device.state_topic_buffer.clear();
                    write!(device.state_topic_buffer, "{state_topic_display}")
                        .expect("state topic buffer too small");
                    device.state_topic_buffer.as_str()
                }
            };

            match embassy_time::with_timeout(
                MQTT_TIMEOUT,
                client.publish(publish_topic, device.publish_buffer),
            )
            .await
            {
                Ok(Ok(_)) => {}
                Ok(Err(err)) => {
                    crate::log::error!(
                        "mqtt state publish on topic '{}' failed with: {:?}",
                        publish_topic,
                        crate::log::Debug2Format(&err)
                    );
                    return Err(Error::new("mqtt publish failed"));
                }
                Err(_) => {
                    crate::log::error!("mqtt state publish on topic '{}' timed out", publish_topic);
                    return Err(Error::new("mqtt publish timed out"));
                }
            }
        }
        first_iteration_push = false;

        let receive = client.receive();
        let waker = wait_on_atomic_waker(device.waker);
        let publish =
            match embassy_futures::select::select3(receive, waker, ping_ticker.next()).await {
                embassy_futures::select::Either3::First(packet) => match packet {
                    Ok(mqtt::Packet::Publish(publish)) => publish,
                    Err(err) => {
                        crate::log::error!(
                            "mqtt receive failed with: {:?}",
                            crate::log::Debug2Format(&err)
                        );
                        return Err(Error::new("mqtt receive failed"));
                    }
                    _ => continue,
                },
                embassy_futures::select::Either3::Second(_) => continue,
                embassy_futures::select::Either3::Third(_) => {
                    if let Err(err) = client.ping().await {
                        crate::log::error!(
                            "mqtt ping failed with: {:?}",
                            crate::log::Debug2Format(&err)
                        );
                        return Err(Error::new("mqtt ping failed"));
                    }
                    continue;
                }
            };

        let entity = 'entity_search_block: {
            for entity in device.entities {
                let mut data = entity.borrow_mut();
                let data = match data.as_mut() {
                    Some(data) => data,
                    None => break,
                };

                let command_topic_display = CommandTopicDisplay {
                    device_id: device.config.device_id,
                    entity_id: data.config.id,
                };
                device.command_topic_buffer.clear();
                write!(device.command_topic_buffer, "{command_topic_display}")
                    .expect("command topic buffer too small");

                if device.command_topic_buffer.as_bytes() == publish.topic.as_bytes() {
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
        match embassy_time::with_timeout(
            MQTT_TIMEOUT,
            client.receive_data(&mut read_buffer[..data_len]),
        )
        .await
        {
            Ok(Ok(())) => {}
            Ok(Err(err)) => {
                crate::log::error!(
                    "mqtt receive data failed with: {:?}",
                    crate::log::Debug2Format(&err)
                );
                return Err(Error::new("mqtt receive data failed"));
            }
            Err(_) => {
                crate::log::error!("mqtt receive data timed out");
                return Err(Error::new("mqtt receive data timed out"));
            }
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

pub async fn connect_and_run(
    stack: embassy_net::Stack<'_>,
    mut device: Device<'_>,
    address: &str,
) -> ! {
    const DEFAULT_MQTT_PORT: u16 = 1883;

    let mut rx_buffer = [0u8; 1024];
    let mut tx_buffer = [0u8; 1024];
    let mut delay = false;

    loop {
        if !delay {
            delay = true;
        } else {
            crate::log::info!("Retrying connection in 5 seconds...");
            Timer::after_secs(5).await;
        }

        let addr = {
            // Try to parse as complete SocketAddrV4 first (e.g., "192.168.1.1:1883")
            if let Ok(sock_addr) = address.parse::<SocketAddrV4>() {
                sock_addr
            }
            // Try to parse as Ipv4Addr with default port (e.g., "192.168.1.1")
            else if let Ok(ip_addr) = address.parse::<Ipv4Addr>() {
                SocketAddrV4::new(ip_addr, DEFAULT_MQTT_PORT)
            }
            // Otherwise, parse as hostname:port or hostname
            else {
                let (addr_str, port) = match address.split_once(':') {
                    Some((addr_str, port_str)) => {
                        let port = port_str
                            .parse::<u16>()
                            .expect("Invalid port number in address");
                        (addr_str, port)
                    }
                    None => (address, DEFAULT_MQTT_PORT),
                };

                let addrs = match stack
                    .dns_query(addr_str, embassy_net::dns::DnsQueryType::A)
                    .await
                {
                    Ok(addrs) => addrs,
                    Err(err) => {
                        crate::log::error!(
                            "DNS query for '{}' failed with: {:?}",
                            addr_str,
                            crate::log::Debug2Format(&err)
                        );
                        continue;
                    }
                };

                #[allow(unreachable_patterns)]
                let ipv4_addr = match addrs
                    .iter()
                    .filter_map(|addr| match addr {
                        embassy_net::IpAddress::Ipv4(ipv4) => Some(*ipv4),
                        _ => None,
                    })
                    .next()
                {
                    Some(addr) => addr,
                    None => {
                        crate::log::error!(
                            "DNS query for '{}' returned no IPv4 addresses",
                            addr_str
                        );
                        continue;
                    }
                };

                SocketAddrV4::new(ipv4_addr, port)
            }
        };

        crate::log::info!("Connecting to MQTT broker at {}", addr);

        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

        let connect_fut = embassy_time::with_timeout(Duration::from_secs(10), socket.connect(addr));
        match connect_fut.await {
            Ok(Err(err)) => {
                crate::log::error!(
                    "TCP connect to {} failed with: {:?}",
                    addr,
                    crate::log::Debug2Format(&err)
                );
                continue;
            }
            Err(_) => {
                crate::log::error!("TCP connect to {} timed out", addr);
                continue;
            }
            _ => {}
        }

        socket.set_timeout(None);

        if let Err(err) = run(&mut device, &mut socket).await {
            crate::log::error!(
                "Device run failed with: {:?}",
                crate::log::Debug2Format(&err)
            );
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
