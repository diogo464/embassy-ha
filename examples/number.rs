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

    let number = device.create_number(
        "number-id",
        embassy_ha::NumberConfig {
            common: embassy_ha::EntityCommonConfig {
                name: Some("Number Name"),
                ..Default::default()
            },
            unit: Some(embassy_ha::NumberUnit::Meter),
            min: Some(0.0),
            max: Some(20.0),
            step: Some(0.5),
            mode: embassy_ha::NumberMode::Slider,
            class: embassy_ha::NumberClass::Distance,
        },
    );

    spawner.must_spawn(number_task(number));

    device.run(&mut stream).await;
}

#[embassy_executor::task]
async fn number_task(mut number: embassy_ha::Number<'static>) {
    loop {
        let value = number.wait().await;
        number.set(value);
        Timer::after_secs(1).await;
    }
}

example_main!();
