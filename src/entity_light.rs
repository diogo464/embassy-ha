use crate::{CommandPolicy, Entity, EntityCommonConfig, EntityConfig, constants};

#[derive(Debug, Clone, Copy, Default)]
pub struct LightColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Default)]
pub struct LightState {
    pub on: bool,
    pub brightness: Option<u8>,
    pub color_temp: Option<u16>,
    pub color: Option<LightColor>,
}

#[derive(Debug, Clone, Default)]
pub struct LightCommand {
    pub state: Option<bool>,
    pub brightness: Option<u8>,
    pub color_temp: Option<u16>,
    pub color: Option<LightColor>,
}

#[derive(Debug, Default)]
pub struct LightConfig {
    pub common: EntityCommonConfig,
    pub brightness: bool,
    pub color_temp: bool,
    pub rgb: bool,
    pub min_mireds: Option<u16>,
    pub max_mireds: Option<u16>,
    pub command_policy: CommandPolicy,
}

impl LightConfig {
    pub(crate) fn populate(&self, config: &mut EntityConfig) {
        self.common.populate(config);
        config.domain = constants::HA_DOMAIN_LIGHT;
        config.schema = Some("json");
        config.light_brightness = self.brightness;
        config.light_color_temp = self.color_temp;
        config.light_rgb = self.rgb;
        config.light_min_mireds = self.min_mireds;
        config.light_max_mireds = self.max_mireds;
        config.light_color_modes = Some(match (self.rgb, self.color_temp, self.brightness) {
            (true, true, _)      => &["rgb", "color_temp"],
            (true, false, _)     => &["rgb"],
            (false, true, _)     => &["color_temp"],
            (false, false, true) => &["brightness"],
            (false, false, false) => &["onoff"],
        });
    }
}

pub struct Light<'a>(Entity<'a>);

impl<'a> Light<'a> {
    pub(crate) fn new(entity: Entity<'a>) -> Self {
        Self(entity)
    }

    pub fn state(&self) -> Option<LightState> {
        self.0.with_data(|data| data.storage.as_light_mut().state.clone())
    }

    pub fn command(&self) -> Option<LightCommand> {
        self.0.with_data(|data| data.storage.as_light_mut().command.clone())
    }

    pub fn set(&mut self, state: LightState) {
        self.0.with_data(|data| {
            data.storage.as_light_mut().state = Some(state);
        });
        self.0.queue_publish();
    }

    pub async fn wait(&mut self) -> LightCommand {
        loop {
            self.0.wait_command().await;
            if let Some(command) = self.command() {
                return command;
            }
        }
    }
}
