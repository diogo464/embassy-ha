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

    let sensor = device.create_binary_sensor(
        "binary-sensor-id",
        "Binary Sensor",
        embassy_ha::constants::HA_DEVICE_CLASS_BINARY_SENSOR_SMOKE,
    );

    spawner.must_spawn(binary_sensor_class(sensor));

    device.run(&mut stream).await;
}

#[embassy_executor::task]
async fn binary_sensor_class(mut switch: embassy_ha::BinarySensor<'static>) {
    loop {
        let state = switch.toggle();
        println!("state = {}", state);
        Timer::after_secs(2).await;
    }
}

example_main!();
