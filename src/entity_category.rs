use crate::constants;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityCategory {
    Config,
    Diagnostic,
}

impl EntityCategory {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            EntityCategory::Config => constants::HA_ENTITY_CATEGORY_CONFIG,
            EntityCategory::Diagnostic => constants::HA_ENTITY_CATEGORY_DIAGNOSTIC,
        }
    }
}

