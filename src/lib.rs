#![no_std]

use core::{cell::RefCell, task::Waker};

use defmt::Format;
use embassy_sync::waitqueue::AtomicWaker;
use embassy_time::Timer;
use heapless::{
    Vec, VecView,
    string::{String, StringView},
};
use serde::Serialize;

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

#[derive(Debug, Format, Clone, Copy, Serialize)]
struct DeviceDiscovery<'a> {
    identifiers: &'a [&'a str],
    name: &'a str,
    manufacturer: &'a str,
    model: &'a str,
}

#[derive(Debug, Format, Serialize)]
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
            discovery_topic_buffer: Default::default(),
            state_topic_buffer: Default::default(),
            command_topic_buffer: Default::default(),
        }
    }
}

struct EntityData {
    config: EntityConfig,
    publish_dirty: bool,
    publish_value: heapless::Vec<u8, 64>,
    command_dirty: bool,
    command_value: heapless::Vec<u8, 64>,
    command_wait_waker: Option<Waker>,
    command_instant: Option<embassy_time::Instant>,
}

pub struct Entity<'a> {
    pub(crate) data: &'a RefCell<Option<EntityData>>,
    pub(crate) waker: &'a AtomicWaker,
}

impl<'a> Entity<'a> {
    pub fn publish(&mut self, payload: &[u8]) {
        self.publish_with(|view| view.extend_from_slice(payload).unwrap());
    }

    pub fn publish_with<F>(&mut self, f: F)
    where
        F: FnOnce(&mut VecView<u8>),
    {
        self.with_data(move |data| {
            data.publish_value.clear();
            f(data.publish_value.as_mut_view());
            data.publish_dirty = true;
        });
        self.waker.wake();
    }

    pub fn publish_str(&mut self, payload: &str) {
        self.publish(payload.as_bytes());
    }

    pub fn publish_display(&mut self, payload: &impl core::fmt::Display) {
        use core::fmt::Write;

        self.publish_with(|view| {
            view.clear();
            write!(view, "{}", payload).unwrap();
        });
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
                    let dirty = data.command_dirty;
                    if dirty {
                        data.command_dirty = false;
                        data.command_wait_waker = None;
                        core::task::Poll::Ready(())
                    } else {
                        // TODO: avoid clone if waker would wake
                        data.command_wait_waker = Some(cx.waker().clone());
                        core::task::Poll::Pending
                    }
                })
            }
        }

        Fut(self).await
    }

    pub fn with_command<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        self.with_data(|data| f(data.command_value.as_slice()))
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
            discovery_topic_buffer: &mut resources.discovery_topic_buffer,
            state_topic_buffer: &mut resources.state_topic_buffer,
            command_topic_buffer: &mut resources.command_topic_buffer,
        }
    }

    pub fn create_entity(&self, config: EntityConfig) -> Entity<'a> {
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
            publish_dirty: false,
            publish_value: Default::default(),
            command_dirty: false,
            command_value: Default::default(),
            command_wait_waker: None,
            command_instant: None,
        };
        self.entities[index].replace(Some(data));

        Entity {
            data: &self.entities[index],
            waker: self.waker,
        }
    }

    pub fn create_temperature_sensor(
        &self,
        id: &'static str,
        config: TemperatureSensorConfig,
    ) -> TemperatureSensor<'a> {
        let mut entity_config = EntityConfig::default();
        entity_config.id = id;
        config.populate(&mut entity_config);

        let entity = self.create_entity(entity_config);
        TemperatureSensor::new(entity)
    }

    pub fn create_button(&self, id: &'static str, config: ButtonConfig) -> Button<'a> {
        let mut entity_config = EntityConfig::default();
        entity_config.id = id;
        config.populate(&mut entity_config);

        let entity = self.create_entity(entity_config);
        Button::new(entity)
    }

    pub fn create_number(&self, id: &'static str, config: NumberConfig) -> Number<'a> {
        let mut entity_config = EntityConfig::default();
        entity_config.id = id;
        config.populate(&mut entity_config);

        let entity = self.create_entity(entity_config);
        Number::new(entity)
    }

    pub fn create_switch(&self, id: &'static str, config: SwitchConfig) -> Switch<'a> {
        let mut entity_config = EntityConfig::default();
        entity_config.id = id;
        config.populate(&mut entity_config);

        let entity = self.create_entity(entity_config);
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

        let entity = self.create_entity(entity_config);
        BinarySensor::new(entity)
    }

    pub async fn run<T: Transport>(&mut self, transport: &mut T) -> ! {
        loop {
            self.run_iteration(&mut *transport).await;
            Timer::after_millis(5000).await;
        }
    }

    async fn run_iteration<T: Transport>(&mut self, transport: T) {
        let mut client = embedded_mqtt::Client::new(self.mqtt_resources, transport);
        client.connect(self.config.device_id).await.unwrap();

        defmt::info!("sending discover messages");
        let device_discovery = DeviceDiscovery {
            identifiers: &[self.config.device_id],
            name: self.config.device_name,
            manufacturer: self.config.manufacturer,
            model: self.config.model,
        };

        for entity in self.entities {
            use core::fmt::Write;

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

                write!(
                    self.discovery_topic_buffer,
                    "{}",
                    DiscoveryTopicDisplay {
                        domain: entity_config.domain,
                        device_id: self.config.device_id,
                        entity_id: entity_config.id,
                    }
                )
                .unwrap();

                write!(
                    self.state_topic_buffer,
                    "{}",
                    StateTopicDisplay {
                        device_id: self.config.device_id,
                        entity_id: entity_config.id
                    }
                )
                .unwrap();

                write!(
                    self.command_topic_buffer,
                    "{}",
                    CommandTopicDisplay {
                        device_id: self.config.device_id,
                        entity_id: entity_config.id
                    }
                )
                .unwrap();

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
                    device: &device_discovery,
                };
                defmt::info!("discovery: {}", discovery);

                self.discovery_buffer
                    .resize(self.discovery_buffer.capacity(), 0)
                    .unwrap();
                let n = serde_json_core::to_slice(&discovery, &mut self.discovery_buffer).unwrap();
                self.discovery_buffer.truncate(n);
            }

            defmt::info!(
                "sending discovery to {}",
                self.discovery_topic_buffer.as_str()
            );
            client
                .publish(&self.discovery_topic_buffer, &self.discovery_buffer)
                .await
                .unwrap();
            client.subscribe(&self.command_topic_buffer).await.unwrap();
        }

        loop {
            use core::fmt::Write;

            for entity in self.entities {
                {
                    let mut entity = entity.borrow_mut();
                    let entity = match entity.as_mut() {
                        Some(entity) => entity,
                        None => break,
                    };

                    if !entity.publish_dirty {
                        continue;
                    }

                    entity.publish_dirty = false;

                    self.state_topic_buffer.clear();
                    write!(
                        self.state_topic_buffer,
                        "{}",
                        StateTopicDisplay {
                            device_id: self.config.device_id,
                            entity_id: entity.config.id
                        }
                    )
                    .unwrap();

                    self.publish_buffer.clear();
                    self.publish_buffer
                        .extend_from_slice(entity.publish_value.as_slice())
                        .unwrap();
                }

                client
                    .publish(&self.state_topic_buffer, self.publish_buffer)
                    .await
                    .unwrap();
            }

            let receive = client.receive();
            let waker = wait_on_atomic_waker(self.waker);
            match embassy_futures::select::select(receive, waker).await {
                embassy_futures::select::Either::First(packet) => {
                    let packet = packet.unwrap();
                    let mut read_buffer = [0u8; 128];
                    if let embedded_mqtt::Packet::Publish(publish) = packet {
                        if publish.data_len > 128 {
                            defmt::warn!("mqtt publish payload too large, ignoring message");
                        } else {
                            let b = &mut read_buffer[..publish.data_len];
                            client.receive_data(b).await.unwrap();
                            defmt::info!("receive value {}", str::from_utf8(b).unwrap());
                            for entity in self.entities {
                                let mut entity = entity.borrow_mut();
                                if let Some(entity) = entity.as_mut() {
                                    entity.command_dirty = true;
                                    entity.command_value.clear();
                                    entity.command_value.extend_from_slice(b).unwrap();
                                    entity.command_instant = Some(embassy_time::Instant::now());
                                    if let Some(ref waker) = entity.command_wait_waker {
                                        waker.wake_by_ref();
                                    }
                                }
                            }
                        }
                    }
                }
                embassy_futures::select::Either::Second(_) => {}
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

/*
  Step-by-Step Process

  1. What are you measuring/controlling?

  Start with the physical thing:
  - "I want to measure temperature"
  - "I want to detect if a door is open"
  - "I want to control a relay"
  - "I want a button to restart the device"

  2. Pick the component type based on behavior

  Ask yourself:
  - Is it read-only or controllable?
  - Does it have numeric values or on/off states?

  Decision tree:
  Read-only measurement?
  ├─ Numeric value (23.5, 65%, etc.)
  │  └─ Component: sensor
  └─ On/off state (open/closed, detected/not detected)
     └─ Component: binary_sensor

  Controllable?
  ├─ On/off control
  │  └─ Component: switch (or light for LEDs)
  ├─ Adjustable number
  │  └─ Component: number
  ├─ Select from options
  │  └─ Component: select
  └─ Trigger action (no state)
     └─ Component: button

  3. Pick the device_class (if applicable)

  Now look at the component type you chose:

  For sensor - What kind of measurement?
  - Temperature → device_class: "temperature"
  - Humidity → device_class: "humidity"
  - Pressure → device_class: "pressure"
  - Custom metric → device_class: None

  For binary_sensor - What kind of detection?
  - Door → device_class: "door"
  - Motion → device_class: "motion"
  - Window → device_class: "window"
  - Generic → device_class: None

  For button - No device_class needed!

  4. Pick units (if applicable)

  Based on your device_class:
  - Temperature → "°C" or "°F"
  - Humidity → "%"
  - Pressure → "hPa"

  Examples

  Example 1: DHT22 Temperature Reading

  1. What? → Measure temperature
  2. Component? → sensor (numeric, read-only)
  3. Device class? → "temperature"
  4. Unit? → "°C"

  Result:
  - Discovery: homeassistant/sensor/pico2w_temp/config
  - JSON: device_class: "temperature", unit_of_measurement: "°C"

  Example 2: Reed Switch on Door

  1. What? → Detect door open/closed
  2. Component? → binary_sensor (on/off state, read-only)
  3. Device class? → "door"
  4. Unit? → N/A

  Result:
  - Discovery: homeassistant/binary_sensor/pico2w_door/config
  - JSON: device_class: "door"

  Example 3: Relay Control

  1. What? → Control a relay
  2. Component? → switch (on/off, controllable)
  3. Device class? → None (switches typically don't have device_class)
  4. Unit? → N/A

  Result:
  - Discovery: homeassistant/switch/pico2w_relay/config
  - JSON: No device_class needed

  Example 4: Restart Button

  1. What? → Trigger device restart
  2. Component? → button (action trigger, no state)
  3. Device class? → None (buttons don't have device_class)
  4. Unit? → N/A

  Result:
  - Discovery: homeassistant/button/pico2w_restart/config
  - JSON: No device_class, no state_topic

  TL;DR Workflow

  Physical thing
      ↓
  Component type (behavior: read-only numeric? binary? controllable?)
      ↓
  Device class (what specific type?)
      ↓
  Units (if numeric)

  Does this mental model make sense now?
*/
