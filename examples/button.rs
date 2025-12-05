mod common;

use common::AsyncTcp;
use embassy_executor::{Executor, Spawner};
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

    let button = device.create_button("button-sensor-id", embassy_ha::ButtonConfig::default());

    spawner.must_spawn(button_task(button));

    device.run(&mut stream).await;
}

#[embassy_executor::task]
async fn button_task(mut button: embassy_ha::Button<'static>) {
    loop {
        button.pressed().await;
        println!("The button has been pressed");
    }
}

example_main!();
