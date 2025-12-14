# embassy-ha

[![Crates.io](https://img.shields.io/crates/v/embassy-ha.svg)](https://crates.io/crates/embassy-ha)
[![Documentation](https://docs.rs/embassy-ha/badge.svg)](https://docs.rs/embassy-ha)

MQTT Home Assistant integration library for the [Embassy](https://embassy.dev/) async runtime.

## Features

- Support for multiple entity types: sensors, buttons, switches, binary sensors, numbers, device trackers
- Built on top of Embassy's async runtime for embedded systems
- No-std compatible
- Automatic MQTT discovery for Home Assistant
- No runtime allocation

## Installation

```bash
cargo add embassy-ha
```

## Quick Start

This example does not compile as-is because it requires device-specific setup, but it should
be easy to adapt if you already have Embassy running on your microcontroller.

```rust
use embassy_executor::Spawner;
use embassy_ha::{DeviceConfig, SensorConfig, SensorClass, StateClass};
use embassy_time::Timer;
use static_cell::StaticCell;

static HA_RESOURCES: StaticCell<embassy_ha::DeviceResources> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize your network stack
    // This is device specific
    let stack: embassy_net::Stack<'static>;

    // Create a Home Assistant device
    let device = embassy_ha::new(
        HA_RESOURCES.init(Default::default()),
        DeviceConfig {
            device_id: "my-device",
            device_name: "My Device",
            manufacturer: "ACME Corp",
            model: "Model X",
        },
    );

    // Create a temperature sensor
    let sensor_config = SensorConfig {
        class: SensorClass::Temperature,
        state_class: StateClass::Measurement,
        unit: Some(embassy_ha::constants::HA_UNIT_TEMPERATURE_CELSIUS),
        ..Default::default()
    };
    let mut sensor = embassy_ha::create_sensor(&device, "temp-sensor", sensor_config);

    // Spawn the Home Assistant communication task
    spawner.spawn(ha_task(stack, device)).unwrap();

    // Main loop - read and publish temperature
    loop {
        // let temperature = read_temperature().await;
        sensor.publish(temperature);
        Timer::after_secs(60).await;
    }
}

#[embassy_executor::task]
async fn ha_task(stack: embassy_net::Stack<'static>, device: embassy_ha::Device<'static>) {
    embassy_ha::connect_and_run(stack, device, "mqtt-broker-address").await;
}
```

## Examples

The repository includes several examples demonstrating different entity types. To run an example:

```bash
export MQTT_ADDRESS="mqtt://your-mqtt-broker:1883"
cargo run --example sensor
```

Available examples:
- `sensor` - Temperature and humidity sensors
- `button` - Triggerable button entity
- `switch` - On/off switch control
- `binary_sensor` - Binary state sensor
- `number` - Numeric input entity
- `device_tracker` - Location tracking entity

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
