//! MQTT Home Assistant integration library for the [Embassy](https://embassy.dev/) async runtime.
//!
//! # Features
//!
//! - Support for multiple entity types: sensors, buttons, switches, binary sensors, numbers, device trackers
//! - Built on top of Embassy's async runtime for embedded systems
//! - No-std compatible
//! - Automatic MQTT discovery for Home Assistant
//! - No runtime allocation
//!
//! # Installation
//!
//! ```bash
//! cargo add embassy-ha
//! ```
//!
//! # Quick Start
//!
//! This example does not compile as-is because it requires device-specific setup, but it should
//! be easy to adapt if you already have Embassy running on your microcontroller.
//!
//! ```no_run
//! use embassy_executor::Spawner;
//! use embassy_ha::{DeviceConfig, SensorConfig, SensorClass, StateClass};
//! use embassy_time::Timer;
//! use static_cell::StaticCell;
//!
//! static HA_RESOURCES: StaticCell<embassy_ha::DeviceResources> = StaticCell::new();
//!
//! #[embassy_executor::main]
//! async fn main(spawner: Spawner) {
//!     // Initialize your network stack
//!     // This is device specific
//!     let stack: embassy_net::Stack<'static>;
//! #   let stack = unsafe { core::mem::zeroed() };
//!
//!     // Create a Home Assistant device
//!     let device = embassy_ha::new(
//!         HA_RESOURCES.init(Default::default()),
//!         DeviceConfig {
//!             device_id: "my-device",
//!             device_name: "My Device",
//!             manufacturer: "ACME Corp",
//!             model: "Model X",
//!         },
//!     );
//!
//!     // Create a temperature sensor
//!     let sensor_config = SensorConfig {
//!         class: SensorClass::Temperature,
//!         state_class: StateClass::Measurement,
//!         unit: Some(embassy_ha::constants::HA_UNIT_TEMPERATURE_CELSIUS),
//!         ..Default::default()
//!     };
//!     let mut sensor = embassy_ha::create_sensor(&device, "temp-sensor", sensor_config);
//!
//!     // Spawn the Home Assistant communication task
//!     spawner.spawn(ha_task(stack, device)).unwrap();
//!
//!     // Main loop - read and publish temperature
//!     loop {
//! #       let temperature = 0.0;
//!         // let temperature = read_temperature().await;
//!         sensor.publish(temperature);
//!         Timer::after_secs(60).await;
//!     }
//! }
//!
//! #[embassy_executor::task]
//! async fn ha_task(stack: embassy_net::Stack<'static>, device: embassy_ha::Device<'static>) {
//!     embassy_ha::connect_and_run(stack, device, "mqtt-broker-address").await;
//! }
//! ```
//!
//! # Examples
//!
//! The repository includes several examples demonstrating different entity types. To run an example:
//!
//! ```bash
//! export MQTT_ADDRESS="mqtt://your-mqtt-broker:1883"
//! cargo run --example sensor
//! ```
//!
//! Available examples:
//! - `sensor` - Temperature and humidity sensors
//! - `button` - Triggerable button entity
//! - `switch` - On/off switch control
//! - `binary_sensor` - Binary state sensor
//! - `number` - Numeric input entity
//! - `device_tracker` - Location tracking entity

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

mod command_policy;
pub use command_policy::*;

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
const MQTT_TIMEOUT: Duration = Duration::from_secs(30);

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

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(Format))]
struct EntityIdDiscovery<'a> {
    device_id: &'a str,
    entity_id: &'a str,
}

impl<'a> core::fmt::Display for EntityIdDiscovery<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}_{}", self.device_id, self.entity_id)
    }
}

impl<'a> Serialize for EntityIdDiscovery<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "defmt", derive(Format))]
struct EntityDiscovery<'a> {
    #[serde(rename = "unique_id")]
    id: EntityIdDiscovery<'a>,

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

pub struct DeviceBuffersOwned {
    pub publish: Vec<u8, 2048>,
    pub subscribe: Vec<u8, 128>,
    pub discovery: Vec<u8, 2048>,
    pub availability_topic: String<128>,
    pub discovery_topic: String<128>,
    pub state_topic: String<128>,
    pub command_topic: String<128>,
    pub attributes_topic: String<128>,
}

impl Default for DeviceBuffersOwned {
    fn default() -> Self {
        Self {
            publish: Default::default(),
            subscribe: Default::default(),
            discovery: Default::default(),
            availability_topic: Default::default(),
            discovery_topic: Default::default(),
            state_topic: Default::default(),
            command_topic: Default::default(),
            attributes_topic: Default::default(),
        }
    }
}

impl DeviceBuffersOwned {
    pub fn as_buffers_mut(&mut self) -> DeviceBuffers<'_> {
        DeviceBuffers {
            publish: &mut self.publish,
            subscribe: &mut self.subscribe,
            discovery: &mut self.discovery,
            availability_topic: &mut self.availability_topic,
            discovery_topic: &mut self.discovery_topic,
            state_topic: &mut self.state_topic,
            command_topic: &mut self.command_topic,
            attributes_topic: &mut self.attributes_topic,
        }
    }
}

pub struct DeviceBuffers<'a> {
    pub publish: &'a mut VecView<u8>,
    pub subscribe: &'a mut VecView<u8>,
    pub discovery: &'a mut VecView<u8>,
    pub availability_topic: &'a mut StringView,
    pub discovery_topic: &'a mut StringView,
    pub state_topic: &'a mut StringView,
    pub command_topic: &'a mut StringView,
    pub attributes_topic: &'a mut StringView,
}

impl<'a> DeviceBuffers<'a> {
    pub fn clear(&mut self) {
        self.publish.clear();
        self.subscribe.clear();
        self.discovery.clear();
        self.availability_topic.clear();
        self.discovery_topic.clear();
        self.state_topic.clear();
        self.command_topic.clear();
        self.attributes_topic.clear();
    }
}

pub struct DeviceResources {
    waker: AtomicWaker,
    entities: [RefCell<Option<EntityData>>; Self::ENTITY_LIMIT],

    mqtt_resources: mqtt::ClientResources,
    buffers: DeviceBuffersOwned,
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
            buffers: Default::default(),
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
    pub command_policy: CommandPolicy,
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
    pub command_policy: CommandPolicy,
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
    buffers: DeviceBuffers<'a>,
}

pub fn new<'a>(resources: &'a mut DeviceResources, config: DeviceConfig) -> Device<'a> {
    Device {
        config,
        waker: &resources.waker,
        entities: &resources.entities,

        mqtt_resources: &mut resources.mqtt_resources,
        buffers: resources.buffers.as_buffers_mut(),
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
            command_policy: config.command_policy,
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
            command_policy: config.command_policy,
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

async fn device_mqtt_subscribe<T: Transport>(
    client: &mut mqtt::Client<'_, T>,
    topic: impl core::fmt::Display,
) -> Result<(), Error> {
    use core::fmt::Write;

    // Format topic to string for both subscribe call and logging
    let mut topic_buffer = heapless::String::<128>::new();
    write!(&mut topic_buffer, "{}", topic).expect("topic buffer too small");
    let topic_str = topic_buffer.as_str();

    match embassy_time::with_timeout(MQTT_TIMEOUT, client.subscribe(topic_str)).await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(err)) => {
            crate::log::error!(
                "mqtt subscribe to '{}' failed with: {:?}",
                topic_str,
                crate::log::Debug2Format(&err)
            );
            Err(Error::new("mqtt subscribe failed"))
        }
        Err(_) => {
            crate::log::error!("mqtt subscribe to '{}' timed out", topic_str);
            Err(Error::new("mqtt subscribe timed out"))
        }
    }
}

async fn device_mqtt_publish<T: Transport>(
    client: &mut mqtt::Client<'_, T>,
    topic: impl core::fmt::Display,
    data: &[u8],
    retain: bool,
) -> Result<(), Error> {
    use core::fmt::Write;

    // Format topic to string for both publish call and logging
    let mut topic_buffer = heapless::String::<128>::new();
    write!(&mut topic_buffer, "{}", topic).expect("topic buffer too small");
    let topic_str = topic_buffer.as_str();

    let result = if retain {
        embassy_time::with_timeout(
            MQTT_TIMEOUT,
            client.publish_with(
                topic_str,
                data,
                mqtt::PublishParams {
                    retain: true,
                    ..Default::default()
                },
            ),
        )
        .await
    } else {
        embassy_time::with_timeout(MQTT_TIMEOUT, client.publish(topic_str, data)).await
    };

    match result {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(err)) => {
            crate::log::error!(
                "mqtt publish to '{}' failed with: {:?}",
                topic_str,
                crate::log::Debug2Format(&err)
            );
            Err(Error::new("mqtt publish failed"))
        }
        Err(_) => {
            crate::log::error!("mqtt publish to '{}' timed out", topic_str);
            Err(Error::new("mqtt publish timed out"))
        }
    }
}

/// Receives MQTT publish data with timeout and proper error handling.
///
/// This helper function handles the common pattern of receiving MQTT data with:
/// - Automatic timeout handling using MQTT_TIMEOUT
/// - Consistent error logging
/// - Size validation against buffer capacity
///
/// # Arguments
/// * `client` - MQTT client for receiving data
/// * `data_len` - Expected length of data to receive
/// * `buffer` - Buffer to receive the data into
///
/// # Returns
/// * `Ok(&[u8])` - Slice of the buffer containing the received data
/// * `Err(Error)` - If operation fails, times out, or data exceeds buffer size
///
/// # Errors
/// Returns error if:
/// - `data_len` is greater than `buffer.len()` (buffer too small)
/// - The receive operation times out after MQTT_TIMEOUT seconds
/// - The underlying MQTT receive operation fails
async fn mqtt_receive_data<'a, T: Transport>(
    client: &mut mqtt::Client<'_, T>,
    data_len: usize,
    buffer: &'a mut [u8],
) -> Result<&'a [u8], Error> {
    // Validate buffer size - reject if too small (per user requirement)
    if data_len > buffer.len() {
        crate::log::warn!(
            "mqtt publish payload is too large ({} bytes, buffer size {} bytes), rejecting",
            data_len,
            buffer.len()
        );
        return Err(Error::new("mqtt payload too large for buffer"));
    }

    crate::log::debug!("mqtt receiving {} bytes of data", data_len);

    match embassy_time::with_timeout(MQTT_TIMEOUT, client.receive_data(&mut buffer[..data_len]))
        .await
    {
        Ok(Ok(())) => Ok(&buffer[..data_len]),
        Ok(Err(err)) => {
            crate::log::error!(
                "mqtt receive data failed with: {:?}",
                crate::log::Debug2Format(&err)
            );
            Err(Error::new("mqtt receive data failed"))
        }
        Err(_) => {
            crate::log::error!("mqtt receive data timed out");
            Err(Error::new("mqtt receive data timed out"))
        }
    }
}

/// Publishes discovery messages for all entities in the device.
///
/// This function iterates over all entities, generates their discovery messages,
/// publishes them to MQTT, and optionally subscribes to their command topics.
///
/// # Arguments
/// * `client` - MQTT client for publishing and subscribing
/// * `entities` - Slice of entities to publish discoveries for
/// * `buffers` - Device buffers for generating discovery messages
/// * `device_config` - Device configuration
/// * `availability_topic` - The device availability topic string
/// * `subscribe_to_commands` - Whether to subscribe to command topics (true for initial discovery, false for rediscovery)
///
/// # Returns
/// `Ok(())` if all discoveries were published successfully, or an error if any operation fails
async fn publish_entity_discoveries<T: Transport>(
    client: &mut mqtt::Client<'_, T>,
    entities: &[RefCell<Option<EntityData>>],
    buffers: &mut DeviceBuffers<'_>,
    device_config: &DeviceConfig,
    availability_topic: &str,
    subscribe_to_commands: bool,
) -> Result<(), Error> {
    crate::log::debug!("publishing entity discovery messages");

    for entity in entities {
        buffers.clear();

        // Borrow the entity and fill out the buffers to be sent
        // This should be done inside a block so that we do not hold the RefMut across an await
        {
            let mut entity = entity.borrow_mut();
            let entity = match entity.as_mut() {
                Some(entity) => entity,
                None => break,
            };

            generate_entity_discovery(buffers, device_config, &entity.config, availability_topic);
        }

        let discovery_topic = buffers.discovery_topic.as_str();
        crate::log::debug!("sending discovery to topic '{}'", discovery_topic);
        device_mqtt_publish(client, discovery_topic, buffers.discovery, false).await?;

        if subscribe_to_commands {
            let command_topic = buffers.command_topic.as_str();
            crate::log::debug!("subscribing to command topic '{}'", command_topic);
            device_mqtt_subscribe(client, command_topic).await?;
        }
    }

    Ok(())
}

fn generate_entity_discovery(
    buffers: &mut DeviceBuffers<'_>,
    device_config: &DeviceConfig,
    entity_config: &EntityConfig,
    availability_topic: &str,
) {
    use core::fmt::Write;

    let discovery_topic_display = DiscoveryTopicDisplay {
        domain: entity_config.domain,
        device_id: device_config.device_id,
        entity_id: entity_config.id,
    };
    let state_topic_display = StateTopicDisplay {
        device_id: device_config.device_id,
        entity_id: entity_config.id,
    };
    let command_topic_display = CommandTopicDisplay {
        device_id: device_config.device_id,
        entity_id: entity_config.id,
    };
    let attributes_topic_display = AttributesTopicDisplay {
        device_id: device_config.device_id,
        entity_id: entity_config.id,
    };

    write!(buffers.discovery_topic, "{discovery_topic_display}")
        .expect("discovery topic buffer too small");
    write!(buffers.state_topic, "{state_topic_display}").expect("state topic buffer too small");
    write!(buffers.command_topic, "{command_topic_display}")
        .expect("command topic buffer too small");
    write!(buffers.attributes_topic, "{attributes_topic_display}")
        .expect("attributes topic buffer too small");

    let device_discovery = DeviceDiscovery {
        identifiers: &[device_config.device_id],
        name: device_config.device_name,
        manufacturer: device_config.manufacturer,
        model: device_config.model,
    };

    let discovery = EntityDiscovery {
        id: EntityIdDiscovery {
            device_id: device_config.device_id,
            entity_id: entity_config.id,
        },
        name: entity_config.name,
        device_class: entity_config.device_class,
        state_topic: Some(buffers.state_topic.as_str()),
        command_topic: Some(buffers.command_topic.as_str()),
        json_attributes_topic: Some(buffers.attributes_topic.as_str()),
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

    buffers
        .discovery
        .resize(buffers.discovery.capacity(), 0)
        .unwrap();
    let n = serde_json_core::to_slice(&discovery, buffers.discovery)
        .expect("discovery buffer too small");
    buffers.discovery.truncate(n);
}

/// Runs the main Home Assistant device event loop.
///
/// This function handles MQTT communication, entity discovery, and state updates. It will run
/// until the first error is encountered, at which point it returns immediately.
///
/// # Behavior
///
/// - Connects to the MQTT broker using the provided transport
/// - Publishes discovery messages for all entities
/// - Subscribes to command topics for controllable entities
/// - Enters the main event loop to handle state updates and commands
/// - Returns on the first error (connection loss, timeout, protocol error, etc.)
///
/// # Error Handling
///
/// This function should be called inside a retry loop, as any network error will cause this
/// function to fail. When an error occurs, the transport may be in an invalid state and should
/// be re-established before calling `run` again.
///
/// # Example
///
/// ```no_run
/// # use embassy_ha::{Device, Transport};
/// # async fn example(mut device: Device<'_>, create_transport: impl Fn() -> impl Transport) {
/// loop {
///     let mut transport = create_transport();
///
///     match embassy_ha::run(&mut device, &mut transport).await {
///         Ok(()) => {
///             // Normal exit (this shouldn't happen in practice)
///             break;
///         }
///         Err(err) => {
///             // Log error and retry after delay
///             // The transport connection should be re-established
///             embassy_time::Timer::after_secs(5).await;
///         }
///     }
/// }
/// # }
/// ```
///
/// For a higher-level alternative that handles retries automatically, see [`connect_and_run`].
pub async fn run<T: Transport>(device: &mut Device<'_>, transport: &mut T) -> Result<(), Error> {
    use core::fmt::Write;

    device.buffers.availability_topic.clear();
    write!(
        device.buffers.availability_topic,
        "{}",
        DeviceAvailabilityTopic {
            device_id: device.config.device_id
        }
    )
    .expect("device availability buffer too small");

    // Store availability_topic in a separate buffer to avoid borrow conflicts
    let mut availability_topic_copy = heapless::String::<128>::new();
    availability_topic_copy
        .push_str(device.buffers.availability_topic.as_str())
        .expect("availability topic too large");
    let availability_topic = availability_topic_copy.as_str();

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

    device_mqtt_subscribe(&mut client, constants::HA_STATUS_TOPIC).await?;

    publish_entity_discoveries(
        &mut client,
        device.entities,
        &mut device.buffers,
        &device.config,
        availability_topic,
        true,
    )
    .await?;

    device_mqtt_publish(
        &mut client,
        availability_topic,
        AVAILABLE_PAYLOAD.as_bytes(),
        true,
    )
    .await?;

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
                device.buffers.publish.clear();

                let mut publish_to_attributes = false;
                match &entity.storage {
                    EntityStorage::Switch(SwitchStorage {
                        state: Some(SwitchState { value, .. }),
                        ..
                    }) => device
                        .buffers
                        .publish
                        .extend_from_slice(value.as_str().as_bytes())
                        .expect("publish buffer too small for switch state payload"),
                    EntityStorage::BinarySensor(BinarySensorStorage {
                        state: Some(BinarySensorState { value, .. }),
                    }) => device
                        .buffers
                        .publish
                        .extend_from_slice(value.as_str().as_bytes())
                        .expect("publish buffer too small for binary sensor state payload"),
                    EntityStorage::NumericSensor(NumericSensorStorage {
                        state: Some(NumericSensorState { value, .. }),
                        ..
                    }) => write!(device.buffers.publish, "{}", value)
                        .expect("publish buffer too small for numeric sensor payload"),
                    EntityStorage::Number(NumberStorage {
                        state: Some(NumberState { value, .. }),
                        ..
                    }) => write!(device.buffers.publish, "{}", value)
                        .expect("publish buffer too small for number state payload"),
                    EntityStorage::DeviceTracker(DeviceTrackerStorage {
                        state: Some(tracker_state),
                    }) => {
                        publish_to_attributes = true;
                        device
                            .buffers
                            .publish
                            .resize(device.buffers.publish.capacity(), 0)
                            .expect("resize to capacity should never fail");
                        let n = serde_json_core::to_slice(&tracker_state, device.buffers.publish)
                            .expect("publish buffer too small for tracker state payload");
                        device.buffers.publish.truncate(n);
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
                    device.buffers.attributes_topic.clear();
                    write!(
                        device.buffers.attributes_topic,
                        "{attributes_topic_display}"
                    )
                    .expect("attributes topic buffer too small");
                    device.buffers.attributes_topic.as_str()
                } else {
                    let state_topic_display = StateTopicDisplay {
                        device_id: device.config.device_id,
                        entity_id: entity.config.id,
                    };
                    device.buffers.state_topic.clear();
                    write!(device.buffers.state_topic, "{state_topic_display}")
                        .expect("state topic buffer too small");
                    device.buffers.state_topic.as_str()
                }
            };

            device_mqtt_publish(&mut client, publish_topic, device.buffers.publish, false).await?;
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

        if publish.topic == constants::HA_STATUS_TOPIC {
            let mut receive_buffer = [0u8; 64];
            let receive_data_len = publish.data_len;
            let receive_data =
                mqtt_receive_data(&mut client, receive_data_len, &mut receive_buffer).await?;

            if receive_data == constants::HA_STATUS_PAYLOAD_ONLINE.as_bytes() {
                first_iteration_push = true;

                crate::log::debug!("home assistant came online, republishing discoveries");
                publish_entity_discoveries(
                    &mut client,
                    device.entities,
                    &mut device.buffers,
                    &device.config,
                    availability_topic,
                    false,
                )
                .await?;
            }
            continue;
        }

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
                device.buffers.command_topic.clear();
                write!(device.buffers.command_topic, "{command_topic_display}")
                    .expect("command topic buffer too small");

                if device.buffers.command_topic.as_bytes() == publish.topic.as_bytes() {
                    break 'entity_search_block entity;
                }
            }
            continue 'outer_loop;
        };

        let mut read_buffer = [0u8; 128];
        let data_len = publish.data_len;
        let receive_data = match mqtt_receive_data(&mut client, data_len, &mut read_buffer).await {
            Ok(data) => data,
            Err(_) => continue 'outer_loop,
        };

        let command = match str::from_utf8(receive_data) {
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
                if switch_storage.command_policy == CommandPolicy::PublishState {
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
                if number_storage.command_policy == CommandPolicy::PublishState {
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

/// High-level function that manages TCP connections and runs the device event loop with automatic retries.
///
/// This is a convenience wrapper around [`run`] that handles:
/// - DNS resolution (if hostname is provided)
/// - TCP connection establishment
/// - Automatic reconnection on failure with 5-second delay
/// - Infinite retry loop
///
/// # Arguments
///
/// * `stack` - The Embassy network stack for TCP connections
/// * `device` - The Home Assistant device to run
/// * `address` - MQTT broker address in one of these formats:
///   - `"192.168.1.100"` - IPv4 address (uses default port 1883)
///   - `"192.168.1.100:1883"` - IPv4 address with explicit port
///   - `"mqtt.example.com"` - Hostname (uses default port 1883)
///   - `"mqtt.example.com:1883"` - Hostname with explicit port
///
/// # Returns
///
/// This function never returns normally (returns `!`). It runs indefinitely, automatically
/// reconnecting on any error.
///
/// # Example
///
/// ```no_run
/// # use embassy_executor::Spawner;
/// # use embassy_ha::{Device, DeviceConfig};
/// # use static_cell::StaticCell;
/// # static HA_RESOURCES: StaticCell<embassy_ha::DeviceResources> = StaticCell::new();
/// #[embassy_executor::task]
/// async fn ha_task(stack: embassy_net::Stack<'static>) {
///     let device = embassy_ha::new(
///         HA_RESOURCES.init(Default::default()),
///         DeviceConfig {
///             device_id: "my-device",
///             device_name: "My Device",
///             manufacturer: "ACME",
///             model: "X",
///         },
///     );
///
///     // This function never returns
///     embassy_ha::connect_and_run(stack, device, "mqtt.example.com:1883").await;
/// }
/// ```
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
