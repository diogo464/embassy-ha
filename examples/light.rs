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
            device_id: "example-light-device",
            device_name: "Example Light Device",
            manufacturer: "Example Manufacturer",
            model: "Example Model",
        },
    );

    let light = embassy_ha::create_light(
        &device,
        "light-id",
        embassy_ha::LightConfig {
            common: embassy_ha::EntityCommonConfig {
                name: Some("Example Light"),
                ..Default::default()
            },
            brightness: true,
            color_temp: true,
            rgb: true,
            min_mireds: Some(153),
            max_mireds: Some(500),
            ..Default::default()
        },
    );

    spawner.must_spawn(light_task(light));

    embassy_ha::run(&mut device, &mut stream).await.unwrap();
}

#[embassy_executor::task]
async fn light_task(mut light: embassy_ha::Light<'static>) {
    // Publish initial state: off, with a default brightness and color_temp so HA
    // knows which controls to show
    light.set(embassy_ha::LightState {
        on: false,
        brightness: Some(255),
        color_temp: Some(370),
        ..Default::default()
    });

    loop {
        let command = light.wait().await;
        tracing::info!(?command, "received light command");

        let current = light.state().unwrap_or_default();
        let new_on = command.state.unwrap_or(current.on);
        let new_brightness = command.brightness.or(current.brightness);
        let new_color_temp = command.color_temp.or(current.color_temp);
        let new_color = command.color.or(current.color);

        let new_state = embassy_ha::LightState {
            on: new_on,
            brightness: new_brightness,
            color_temp: new_color_temp,
            color: new_color,
        };

        tracing::info!(
            on = new_state.on,
            brightness = ?new_state.brightness,
            color_temp = ?new_state.color_temp,
            color = ?new_state.color,
            "applying new light state"
        );

        light.set(new_state);
    }
}

example_main!();
