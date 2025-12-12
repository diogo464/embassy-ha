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

    let tracker = embassy_ha::create_device_tracker(
        &device,
        "device-tracker-id",
        embassy_ha::DeviceTrackerConfig {
            common: embassy_ha::EntityCommonConfig {
                name: Some("Device Tracker Name"),
                ..Default::default()
            },
        },
    );

    spawner.must_spawn(tracker_task(tracker));

    embassy_ha::run(&mut device, &mut stream).await.unwrap();
}

#[embassy_executor::task]
async fn tracker_task(mut tracker: embassy_ha::DeviceTracker<'static>) {
    let locations = [
        embassy_ha::DeviceTrackerLocation {
            latitude: 38.72197768549349,
            longitude: -9.195954862428767,
            accuracy: None,
        },
        embassy_ha::DeviceTrackerLocation {
            latitude: 38.72253035645279,
            longitude: -9.179484976517816,
            accuracy: None,
        },
        embassy_ha::DeviceTrackerLocation {
            latitude: 38.72962258768138,
            longitude: -9.195895830579625,
            accuracy: None,
        },
    ];

    let mut idx = 0;
    loop {
        tracker.publish(locations[idx]);
        idx = (idx + 1) % locations.len();
        Timer::after_secs(1).await;
    }
}

example_main!();
