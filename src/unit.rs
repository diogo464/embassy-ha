#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celcius,
    Kelvin,
    Fahrenheit,
    Other(&'static str),
}

impl TemperatureUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            TemperatureUnit::Celcius => crate::constants::HA_UNIT_TEMPERATURE_CELSIUS,
            TemperatureUnit::Kelvin => crate::constants::HA_UNIT_TEMPERATURE_KELVIN,
            TemperatureUnit::Fahrenheit => crate::constants::HA_UNIT_TEMPERATURE_FAHRENHEIT,
            TemperatureUnit::Other(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumidityUnit {
    Percentage,
    Other(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryUnit {
    Percentage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightUnit {
    Lux,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureUnit {
    HectoPascal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnergyUnit {
    KiloWattHour,
}
