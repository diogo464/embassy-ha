use crate::{Entity, EntityCommonConfig, EntityConfig, TemperatureUnit, constants};

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
        use core::fmt::Write;
        self.0
            .publish_with(|view| write!(view, "{}", temperature).unwrap());
    }
}
