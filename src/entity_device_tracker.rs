use crate::{DeviceTrackerState, Entity, EntityCommonConfig, EntityConfig, constants};

#[derive(Debug, Default)]
pub struct DeviceTrackerConfig {
    pub common: EntityCommonConfig,
}

impl DeviceTrackerConfig {
    pub(crate) fn populate(&self, config: &mut EntityConfig) {
        self.common.populate(config);
        config.domain = constants::HA_DOMAIN_DEVICE_TRACKER;
        config.platform = Some(constants::HA_DOMAIN_DEVICE_TRACKER);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeviceTrackerLocation {
    pub latitude: f32,
    pub longitude: f32,
    pub accuracy: Option<f32>,
}

pub struct DeviceTracker<'a>(Entity<'a>);

impl<'a> DeviceTracker<'a> {
    pub(crate) fn new(entity: Entity<'a>) -> Self {
        Self(entity)
    }

    pub fn publish(&mut self, location: DeviceTrackerLocation) {
        self.0.with_data(|data| {
            let storage = data.storage.as_device_tracker_mut();
            storage.state = Some(DeviceTrackerState {
                latitude: location.latitude,
                longitude: location.longitude,
                gps_accuracy: location.accuracy,
            });
        });
        self.0.queue_publish();
    }
}
