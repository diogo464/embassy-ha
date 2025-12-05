use crate::EntityCategory;

#[derive(Debug, Default)]
pub struct EntityCommonConfig {
    pub name: Option<&'static str>,
    pub icon: Option<&'static str>,
    pub category: Option<EntityCategory>,
    pub picture: Option<&'static str>,
}

impl EntityCommonConfig {
    pub(crate) fn populate(&self, config: &mut EntityConfig) {
        config.name = self.name;
        config.icon = self.icon;
        config.category = self.category.map(|c| c.as_str());
        config.picture = self.picture;
    }
}

#[derive(Default)]
pub struct EntityConfig {
    pub id: &'static str,
    pub name: Option<&'static str>,
    pub domain: &'static str,
    pub device_class: Option<&'static str>,
    pub measurement_unit: Option<&'static str>,
    pub icon: Option<&'static str>,
    pub picture: Option<&'static str>,
    pub category: Option<&'static str>,
    pub state_class: Option<&'static str>,
    pub schema: Option<&'static str>,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub step: Option<f32>,
    pub mode: Option<&'static str>,
}
