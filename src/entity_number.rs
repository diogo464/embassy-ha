use crate::{Entity, EntityCommonConfig, EntityConfig, NumberUnit, constants};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NumberMode {
    #[default]
    Auto,
    Box,
    Slider,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NumberClass {
    #[default]
    Generic,
    ApparentPower,
    Aqi,
    AtmosphericPressure,
    Battery,
    CarbonDioxide,
    CarbonMonoxide,
    Current,
    DataRate,
    DataSize,
    Distance,
    Duration,
    Energy,
    Frequency,
    Gas,
    Humidity,
    Illuminance,
    Irradiance,
    Moisture,
    Monetary,
    NitrogenDioxide,
    NitrogenMonoxide,
    NitrousOxide,
    Ozone,
    Ph,
    Pm1,
    Pm25,
    Pm10,
    PowerFactor,
    Power,
    Precipitation,
    PrecipitationIntensity,
    Pressure,
    ReactivePower,
    SignalStrength,
    SoundPressure,
    Speed,
    SulphurDioxide,
    Temperature,
    VolatileOrganicCompounds,
    VolatileOrganicCompoundsParts,
    Voltage,
    Volume,
    Water,
    Weight,
    WindSpeed,
}

#[derive(Debug)]
pub struct NumberConfig {
    pub common: EntityCommonConfig,
    pub unit: Option<NumberUnit>,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub step: Option<f32>,
    pub mode: NumberMode,
    pub class: NumberClass,
}

impl Default for NumberConfig {
    fn default() -> Self {
        Self {
            common: EntityCommonConfig::default(),
            unit: None,
            min: None,
            max: None,
            step: None,
            mode: NumberMode::Auto,
            class: NumberClass::Generic,
        }
    }
}

impl NumberConfig {
    pub(crate) fn populate(&self, config: &mut EntityConfig) {
        self.common.populate(config);
        config.domain = constants::HA_DOMAIN_NUMBER;
        config.mode = Some(match self.mode {
            NumberMode::Auto => constants::HA_NUMBER_MODE_AUTO,
            NumberMode::Box => constants::HA_NUMBER_MODE_BOX,
            NumberMode::Slider => constants::HA_NUMBER_MODE_SLIDER,
        });
        config.device_class = match self.class {
            NumberClass::Generic => None,
            NumberClass::ApparentPower => Some(constants::HA_DEVICE_CLASS_NUMBER_APPARENT_POWER),
            NumberClass::Aqi => Some(constants::HA_DEVICE_CLASS_NUMBER_AQI),
            NumberClass::AtmosphericPressure => {
                Some(constants::HA_DEVICE_CLASS_NUMBER_ATMOSPHERIC_PRESSURE)
            }
            NumberClass::Battery => Some(constants::HA_DEVICE_CLASS_NUMBER_BATTERY),
            NumberClass::CarbonDioxide => Some(constants::HA_DEVICE_CLASS_NUMBER_CARBON_DIOXIDE),
            NumberClass::CarbonMonoxide => Some(constants::HA_DEVICE_CLASS_NUMBER_CARBON_MONOXIDE),
            NumberClass::Current => Some(constants::HA_DEVICE_CLASS_NUMBER_CURRENT),
            NumberClass::DataRate => Some(constants::HA_DEVICE_CLASS_NUMBER_DATA_RATE),
            NumberClass::DataSize => Some(constants::HA_DEVICE_CLASS_NUMBER_DATA_SIZE),
            NumberClass::Distance => Some(constants::HA_DEVICE_CLASS_NUMBER_DISTANCE),
            NumberClass::Duration => Some(constants::HA_DEVICE_CLASS_NUMBER_DURATION),
            NumberClass::Energy => Some(constants::HA_DEVICE_CLASS_NUMBER_ENERGY),
            NumberClass::Frequency => Some(constants::HA_DEVICE_CLASS_NUMBER_FREQUENCY),
            NumberClass::Gas => Some(constants::HA_DEVICE_CLASS_NUMBER_GAS),
            NumberClass::Humidity => Some(constants::HA_DEVICE_CLASS_NUMBER_HUMIDITY),
            NumberClass::Illuminance => Some(constants::HA_DEVICE_CLASS_NUMBER_ILLUMINANCE),
            NumberClass::Irradiance => Some(constants::HA_DEVICE_CLASS_NUMBER_IRRADIANCE),
            NumberClass::Moisture => Some(constants::HA_DEVICE_CLASS_NUMBER_MOISTURE),
            NumberClass::Monetary => Some(constants::HA_DEVICE_CLASS_NUMBER_MONETARY),
            NumberClass::NitrogenDioxide => {
                Some(constants::HA_DEVICE_CLASS_NUMBER_NITROGEN_DIOXIDE)
            }
            NumberClass::NitrogenMonoxide => {
                Some(constants::HA_DEVICE_CLASS_NUMBER_NITROGEN_MONOXIDE)
            }
            NumberClass::NitrousOxide => Some(constants::HA_DEVICE_CLASS_NUMBER_NITROUS_OXIDE),
            NumberClass::Ozone => Some(constants::HA_DEVICE_CLASS_NUMBER_OZONE),
            NumberClass::Ph => Some(constants::HA_DEVICE_CLASS_NUMBER_PH),
            NumberClass::Pm1 => Some(constants::HA_DEVICE_CLASS_NUMBER_PM1),
            NumberClass::Pm25 => Some(constants::HA_DEVICE_CLASS_NUMBER_PM25),
            NumberClass::Pm10 => Some(constants::HA_DEVICE_CLASS_NUMBER_PM10),
            NumberClass::PowerFactor => Some(constants::HA_DEVICE_CLASS_NUMBER_POWER_FACTOR),
            NumberClass::Power => Some(constants::HA_DEVICE_CLASS_NUMBER_POWER),
            NumberClass::Precipitation => Some(constants::HA_DEVICE_CLASS_NUMBER_PRECIPITATION),
            NumberClass::PrecipitationIntensity => {
                Some(constants::HA_DEVICE_CLASS_NUMBER_PRECIPITATION_INTENSITY)
            }
            NumberClass::Pressure => Some(constants::HA_DEVICE_CLASS_NUMBER_PRESSURE),
            NumberClass::ReactivePower => Some(constants::HA_DEVICE_CLASS_NUMBER_REACTIVE_POWER),
            NumberClass::SignalStrength => Some(constants::HA_DEVICE_CLASS_NUMBER_SIGNAL_STRENGTH),
            NumberClass::SoundPressure => Some(constants::HA_DEVICE_CLASS_NUMBER_SOUND_PRESSURE),
            NumberClass::Speed => Some(constants::HA_DEVICE_CLASS_NUMBER_SPEED),
            NumberClass::SulphurDioxide => Some(constants::HA_DEVICE_CLASS_NUMBER_SULPHUR_DIOXIDE),
            NumberClass::Temperature => Some(constants::HA_DEVICE_CLASS_NUMBER_TEMPERATURE),
            NumberClass::VolatileOrganicCompounds => {
                Some(constants::HA_DEVICE_CLASS_NUMBER_VOLATILE_ORGANIC_COMPOUNDS)
            }
            NumberClass::VolatileOrganicCompoundsParts => {
                Some(constants::HA_DEVICE_CLASS_NUMBER_VOLATILE_ORGANIC_COMPOUNDS_PARTS)
            }
            NumberClass::Voltage => Some(constants::HA_DEVICE_CLASS_NUMBER_VOLTAGE),
            NumberClass::Volume => Some(constants::HA_DEVICE_CLASS_NUMBER_VOLUME),
            NumberClass::Water => Some(constants::HA_DEVICE_CLASS_NUMBER_WATER),
            NumberClass::Weight => Some(constants::HA_DEVICE_CLASS_NUMBER_WEIGHT),
            NumberClass::WindSpeed => Some(constants::HA_DEVICE_CLASS_NUMBER_WIND_SPEED),
        };
        config.measurement_unit = self.unit.as_ref().map(|u| u.as_str());
        config.min = self.min;
        config.max = self.max;
        config.step = self.step;
    }
}

pub struct Number<'a>(Entity<'a>);

impl<'a> Number<'a> {
    pub(crate) fn new(entity: Entity<'a>) -> Self {
        Self(entity)
    }

    pub fn value(&mut self) -> Option<f32> {
        self.0.with_data(|data| {
            str::from_utf8(&data.command_value)
                .ok()
                .and_then(|v| v.parse::<f32>().ok())
        })
    }

    pub async fn value_wait(&mut self) -> f32 {
        loop {
            self.0.wait_command().await;
            match self.value() {
                Some(value) => return value,
                None => continue,
            }
        }
    }

    pub fn value_set(&mut self, value: f32) {
        use core::fmt::Write;
        self.0
            .publish_with(|view| write!(view, "{}", value).unwrap());
    }
}
