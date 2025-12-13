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

    let sensor = embassy_ha::create_binary_sensor(
        &device,
        "binary-sensor-id",
        embassy_ha::BinarySensorConfig {
            common: embassy_ha::EntityCommonConfig {
                name: Some("Binary Sensor"),
                ..Default::default()
            },
            class: embassy_ha::BinarySensorClass::Smoke,
        },
    );

    spawner.spawn(binary_sensor_class(sensor).unwrap());

    embassy_ha::run(&mut device, &mut stream).await.unwrap();
}

#[embassy_executor::task]
async fn binary_sensor_class(mut switch: embassy_ha::BinarySensor<'static>) {
    loop {
        let state = switch.toggle();
        tracing::info!("state = {}", state);
        Timer::after_secs(2).await;
    }
}

example_main!();
