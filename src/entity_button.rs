use crate::{Entity, EntityCommonConfig, EntityConfig, constants};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonClass {
    #[default]
    Generic,
    Identify,
    Restart,
    Update,
}

#[derive(Debug, Default)]
pub struct ButtonConfig {
    pub common: EntityCommonConfig,
    pub class: ButtonClass,
}

impl ButtonConfig {
    pub(crate) fn populate(&self, config: &mut EntityConfig) {
        self.common.populate(config);
        config.domain = constants::HA_DOMAIN_BUTTON;
        config.device_class = match self.class {
            ButtonClass::Generic => None,
            ButtonClass::Identify => Some(constants::HA_DEVICE_CLASS_BUTTON_IDENTIFY),
            ButtonClass::Restart => Some(constants::HA_DEVICE_CLASS_BUTTON_RESTART),
            ButtonClass::Update => Some(constants::HA_DEVICE_CLASS_BUTTON_UPDATE),
        };
    }
}

pub struct Button<'a>(Entity<'a>);

impl<'a> Button<'a> {
    pub(crate) fn new(entity: Entity<'a>) -> Self {
        Self(entity)
    }

    pub async fn pressed(&mut self) {
        self.0.wait_command().await;
    }
}
