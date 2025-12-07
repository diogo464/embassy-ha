mod common;

use common::AsyncTcp;
use embassy_executor::{Executor, Spawner};
use embassy_time::Timer;
use static_cell::StaticCell;

static RESOURCES: StaticCell<embassy_ha::DeviceResources> = StaticCell::new();

#[embassy_executor::task]
async fn main_task(spawner: Spawner) {
    let mut stream = AsyncTcp::connect(std::env!("MQTT_ADDRESS"));

    let mut device = embassy_ha::new(
        RESOURCES.init(Default::default()),
        embassy_ha::DeviceConfig {
            device_id: "example-device-id",
            device_name: "Example Device Name",
            manufacturer: "Example Device Manufacturer",
            model: "Example Device Model",
        },
    );

    let temperature_sensor = embassy_ha::create_sensor(
        &device,
        "random-temperature-sensor-id",
        embassy_ha::SensorConfig {
            common: embassy_ha::EntityCommonConfig {
                name: Some("Random Temperature Sensor"),
                ..Default::default()
            },
            class: embassy_ha::SensorClass::Temperature,
            state_class: embassy_ha::StateClass::Measurement,
            unit: Some(embassy_ha::constants::HA_UNIT_TEMPERATURE_CELSIUS),
            suggested_display_precision: Some(1),
        },
    );

    let humidity_sensor = embassy_ha::create_sensor(
        &device,
        "random-humidity-sensor-id",
        embassy_ha::SensorConfig {
            common: embassy_ha::EntityCommonConfig {
                name: Some("Random Humidity Sensor"),
                ..Default::default()
            },
            class: embassy_ha::SensorClass::Humidity,
            state_class: embassy_ha::StateClass::Measurement,
            unit: Some(embassy_ha::constants::HA_UNIT_PERCENTAGE),
            suggested_display_precision: Some(0),
        },
    );

    spawner.must_spawn(random_temperature_task(temperature_sensor));
    spawner.must_spawn(random_humidity_task(humidity_sensor));

    embassy_ha::run(&mut device, &mut stream).await.unwrap();
}

#[embassy_executor::task]
async fn random_temperature_task(mut sensor: embassy_ha::Sensor<'static>) {
    loop {
        sensor.publish(rand::random_range(0.0..50.0));
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn random_humidity_task(mut sensor: embassy_ha::Sensor<'static>) {
    loop {
        sensor.publish(rand::random_range(0.0..100.0));
        Timer::after_secs(1).await;
    }
}

example_main!();
