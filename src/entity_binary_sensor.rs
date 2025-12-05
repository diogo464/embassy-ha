use crate::{BinarySensorState, BinaryState, Entity, EntityCommonConfig, EntityConfig, constants};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BinarySensorClass {
    #[default]
    Generic,
    Battery,
    BatteryCharging,
    Connectivity,
    Door,
    GarageDoor,
    Motion,
    Occupancy,
    Opening,
    Plug,
    Power,
    Presence,
    Problem,
    Smoke,
    Window,
}

#[derive(Debug, Default)]
pub struct BinarySensorConfig {
    pub common: EntityCommonConfig,
    pub class: BinarySensorClass,
}

impl BinarySensorConfig {
    pub(crate) fn populate(&self, config: &mut EntityConfig) {
        self.common.populate(config);
        config.domain = constants::HA_DOMAIN_BINARY_SENSOR;
        config.device_class = match self.class {
            BinarySensorClass::Generic => None,
            BinarySensorClass::Battery => Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_BATTERY),
            BinarySensorClass::BatteryCharging => {
                Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_BATTERY_CHARGING)
            }
            BinarySensorClass::Connectivity => {
                Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_CONNECTIVITY)
            }
            BinarySensorClass::Door => Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_DOOR),
            BinarySensorClass::GarageDoor => {
                Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_GARAGE_DOOR)
            }
            BinarySensorClass::Motion => Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_MOTION),
            BinarySensorClass::Occupancy => {
                Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_OCCUPANCY)
            }
            BinarySensorClass::Opening => Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_OPENING),
            BinarySensorClass::Plug => Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_PLUG),
            BinarySensorClass::Power => Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_POWER),
            BinarySensorClass::Presence => Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_PRESENCE),
            BinarySensorClass::Problem => Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_PROBLEM),
            BinarySensorClass::Smoke => Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_SMOKE),
            BinarySensorClass::Window => Some(constants::HA_DEVICE_CLASS_BINARY_SENSOR_WINDOW),
        };
    }
}

pub struct BinarySensor<'a>(Entity<'a>);

impl<'a> BinarySensor<'a> {
    pub(crate) fn new(entity: Entity<'a>) -> Self {
        Self(entity)
    }

    pub fn set(&mut self, state: BinaryState) {
        let publish = self.0.with_data(|data| {
            let storage = data.storage.as_binary_sensor_mut();
            let publish = match &storage.state {
                Some(s) => s.value != state,
                None => true,
            };
            storage.state = Some(BinarySensorState {
                value: state,
                timestamp: embassy_time::Instant::now(),
            });
            publish
        });
        if publish {
            self.0.queue_publish();
        }
    }

    pub fn value(&self) -> Option<BinaryState> {
        self.0.with_data(|data| {
            let storage = data.storage.as_binary_sensor_mut();
            storage.state.as_ref().map(|s| s.value)
        })
    }

    pub fn toggle(&mut self) -> BinaryState {
        let new_state = self.value().unwrap_or(BinaryState::Off).flip();
        self.set(new_state);
        new_state
    }
}
