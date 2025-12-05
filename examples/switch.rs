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

    let switch = device.create_switch(
        "switch-id",
        embassy_ha::SwitchConfig {
            common: embassy_ha::EntityCommonConfig {
                name: Some("Example Switch"),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    spawner.must_spawn(switch_task(switch));

    device.run(&mut stream).await;
}

#[embassy_executor::task]
async fn switch_task(mut switch: embassy_ha::Switch<'static>) {
    loop {
        let state = switch.wait().await;
        switch.set(state);

        println!("state = {}", state);
        Timer::after_secs(1).await;
    }
}

example_main!();
