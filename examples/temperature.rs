mod common;

use common::AsyncTcp;
use embassy_executor::{Executor, Spawner};
use embassy_time::Timer;
use static_cell::StaticCell;

static RESOURCES: StaticCell<embassy_ha::DeviceResources> = StaticCell::new();

#[embassy_executor::task]
async fn main_task(spawner: Spawner) {
    let mut stream = AsyncTcp::connect(std::env!("MQTT_ADDRESS"));

    let mut device = embassy_ha::Device::new(
        RESOURCES.init(Default::default()),
        embassy_ha::DeviceConfig {
            device_id: "example-device-id",
            device_name: "Example Device Name",
            manufacturer: "Example Device Manufacturer",
            model: "Example Device Model",
        },
    );

    let constant_temperature_sensor = device.create_temperature_sensor(
        "constant-temperature-sensor-id",
        "Constant Temperature Sensor",
        embassy_ha::TemperatureUnit::Celcius,
    );

    let random_temperature_sensor = device.create_temperature_sensor(
        "random-temperature-sensor-id",
        "Random Temperature Sensor",
        embassy_ha::TemperatureUnit::Celcius,
    );

    spawner.must_spawn(constant_temperature_task(constant_temperature_sensor));
    spawner.must_spawn(random_temperature_task(random_temperature_sensor));

    device.run(&mut stream).await;
}

#[embassy_executor::task]
async fn constant_temperature_task(mut sensor: embassy_ha::TemperatureSensor<'static>) {
    loop {
        sensor.publish(42.0);
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn random_temperature_task(mut sensor: embassy_ha::TemperatureSensor<'static>) {
    loop {
        sensor.publish(rand::random_range(0.0..50.0));
        Timer::after_secs(1).await;
    }
}

example_main!();
