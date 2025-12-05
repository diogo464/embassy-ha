use crate::{BinaryState, Entity, EntityCommonConfig, EntityConfig, SwitchState, constants};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SwitchClass {
    #[default]
    Generic,
    Outlet,
    Switch,
}

#[derive(Debug, Default)]
pub struct SwitchConfig {
    pub common: EntityCommonConfig,
    pub class: SwitchClass,
}

impl SwitchConfig {
    pub(crate) fn populate(&self, config: &mut EntityConfig) {
        self.common.populate(config);
        config.domain = constants::HA_DOMAIN_SWITCH;
        config.device_class = match self.class {
            SwitchClass::Generic => None,
            SwitchClass::Outlet => Some(constants::HA_DEVICE_CLASS_SWITCH_OUTLET),
            SwitchClass::Switch => Some(constants::HA_DEVICE_CLASS_SWITCH_SWITCH),
        };
    }
}

pub struct Switch<'a>(Entity<'a>);

impl<'a> Switch<'a> {
    pub(crate) fn new(entity: Entity<'a>) -> Self {
        Self(entity)
    }

    pub fn state(&self) -> Option<BinaryState> {
        self.0.with_data(|data| {
            let storage = data.storage.as_switch_mut();
            storage.state.as_ref().map(|s| s.value)
        })
    }

    pub fn toggle(&mut self) -> BinaryState {
        let new_state = self.state().unwrap_or(BinaryState::Off).flip();
        self.set(new_state);
        new_state
    }

    pub fn set(&mut self, state: BinaryState) {
        self.0.with_data(|data| {
            let storage = data.storage.as_switch_mut();
            storage.state = Some(SwitchState {
                value: state,
                timestamp: embassy_time::Instant::now(),
            });
        })
    }

    pub async fn wait(&mut self) -> BinaryState {
        loop {
            self.0.wait_command().await;
            if let Some(state) = self.state() {
                return state;
            }
        }
    }
}
