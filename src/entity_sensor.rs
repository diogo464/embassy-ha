use crate::{
    Entity, EntityCommonConfig, EntityConfig, NumericSensorState, TemperatureUnit, constants,
};

#[derive(Debug, Default)]
pub struct TemperatureSensorConfig {
    pub common: EntityCommonConfig,
    pub unit: TemperatureUnit,
}

impl TemperatureSensorConfig {
    pub(crate) fn populate(&self, config: &mut EntityConfig) {
        self.common.populate(config);
        config.domain = constants::HA_DOMAIN_SENSOR;
        config.device_class = Some(constants::HA_DEVICE_CLASS_SENSOR_TEMPERATURE);
        config.measurement_unit = Some(self.unit.as_str());
    }
}

pub struct TemperatureSensor<'a>(Entity<'a>);

impl<'a> TemperatureSensor<'a> {
    pub(crate) fn new(entity: Entity<'a>) -> Self {
        Self(entity)
    }

    pub fn publish(&mut self, temperature: f32) {
        let publish = self.0.with_data(|data| {
            let storage = data.storage.as_numeric_sensor_mut();
            let prev_state = storage.state.replace(NumericSensorState {
                value: temperature,
                timestamp: embassy_time::Instant::now(),
            });
            match prev_state {
                Some(state) => state.value != temperature,
                None => true,
            }
        });
        if publish {
            self.0.queue_publish();
        }
    }
}
