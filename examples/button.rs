mod common;

use common::AsyncTcp;
use embassy_executor::{Executor, Spawner};
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

    let button = embassy_ha::create_button(
        &device,
        "button-sensor-id",
        embassy_ha::ButtonConfig::default(),
    );

    spawner.spawn(button_task(button).unwrap());

    embassy_ha::run(&mut device, &mut stream).await.unwrap();
}

#[embassy_executor::task]
async fn button_task(mut button: embassy_ha::Button<'static>) {
    loop {
        button.pressed().await;
        tracing::info!("The button has been pressed");
    }
}

example_main!();
