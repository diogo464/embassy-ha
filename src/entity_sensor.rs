use crate::{
    Entity, EntityCommonConfig, EntityConfig, NumericSensorState, TemperatureUnit, constants,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StateClass {
    #[default]
    Measurement,
    Total,
    TotalIncreasing,
}

impl StateClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            StateClass::Measurement => constants::HA_STATE_CLASS_MEASUREMENT,
            StateClass::Total => constants::HA_STATE_CLASS_TOTAL,
            StateClass::TotalIncreasing => constants::HA_STATE_CLASS_TOTAL_INCREASING,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorClass {
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
    Date,
    Distance,
    Duration,
    Energy,
    EnergyStorage,
    Enum,
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
    Timestamp,
    VolatileOrganicCompounds,
    VolatileOrganicCompoundsParts,
    Voltage,
    Volume,
    VolumeFlowRate,
    VolumeStorage,
    Water,
    Weight,
    WindSpeed,
    Other(&'static str),
}

impl Default for SensorClass {
    fn default() -> Self {
        SensorClass::Generic
    }
}

impl SensorClass {
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            SensorClass::Generic => None,
            SensorClass::Other(s) => Some(s),
            SensorClass::ApparentPower => Some(constants::HA_DEVICE_CLASS_SENSOR_APPARENT_POWER),
            SensorClass::Aqi => Some(constants::HA_DEVICE_CLASS_SENSOR_AQI),
            SensorClass::AtmosphericPressure => Some(constants::HA_DEVICE_CLASS_SENSOR_ATMOSPHERIC_PRESSURE),
            SensorClass::Battery => Some(constants::HA_DEVICE_CLASS_SENSOR_BATTERY),
            SensorClass::CarbonDioxide => Some(constants::HA_DEVICE_CLASS_SENSOR_CARBON_DIOXIDE),
            SensorClass::CarbonMonoxide => Some(constants::HA_DEVICE_CLASS_SENSOR_CARBON_MONOXIDE),
            SensorClass::Current => Some(constants::HA_DEVICE_CLASS_SENSOR_CURRENT),
            SensorClass::DataRate => Some(constants::HA_DEVICE_CLASS_SENSOR_DATA_RATE),
            SensorClass::DataSize => Some(constants::HA_DEVICE_CLASS_SENSOR_DATA_SIZE),
            SensorClass::Date => Some(constants::HA_DEVICE_CLASS_SENSOR_DATE),
            SensorClass::Distance => Some(constants::HA_DEVICE_CLASS_SENSOR_DISTANCE),
            SensorClass::Duration => Some(constants::HA_DEVICE_CLASS_SENSOR_DURATION),
            SensorClass::Energy => Some(constants::HA_DEVICE_CLASS_SENSOR_ENERGY),
            SensorClass::EnergyStorage => Some(constants::HA_DEVICE_CLASS_SENSOR_ENERGY_STORAGE),
            SensorClass::Enum => Some(constants::HA_DEVICE_CLASS_SENSOR_ENUM),
            SensorClass::Frequency => Some(constants::HA_DEVICE_CLASS_SENSOR_FREQUENCY),
            SensorClass::Gas => Some(constants::HA_DEVICE_CLASS_SENSOR_GAS),
            SensorClass::Humidity => Some(constants::HA_DEVICE_CLASS_SENSOR_HUMIDITY),
            SensorClass::Illuminance => Some(constants::HA_DEVICE_CLASS_SENSOR_ILLUMINANCE),
            SensorClass::Irradiance => Some(constants::HA_DEVICE_CLASS_SENSOR_IRRADIANCE),
            SensorClass::Moisture => Some(constants::HA_DEVICE_CLASS_SENSOR_MOISTURE),
            SensorClass::Monetary => Some(constants::HA_DEVICE_CLASS_SENSOR_MONETARY),
            SensorClass::NitrogenDioxide => Some(constants::HA_DEVICE_CLASS_SENSOR_NITROGEN_DIOXIDE),
            SensorClass::NitrogenMonoxide => Some(constants::HA_DEVICE_CLASS_SENSOR_NITROGEN_MONOXIDE),
            SensorClass::NitrousOxide => Some(constants::HA_DEVICE_CLASS_SENSOR_NITROUS_OXIDE),
            SensorClass::Ozone => Some(constants::HA_DEVICE_CLASS_SENSOR_OZONE),
            SensorClass::Ph => Some(constants::HA_DEVICE_CLASS_SENSOR_PH),
            SensorClass::Pm1 => Some(constants::HA_DEVICE_CLASS_SENSOR_PM1),
            SensorClass::Pm25 => Some(constants::HA_DEVICE_CLASS_SENSOR_PM25),
            SensorClass::Pm10 => Some(constants::HA_DEVICE_CLASS_SENSOR_PM10),
            SensorClass::PowerFactor => Some(constants::HA_DEVICE_CLASS_SENSOR_POWER_FACTOR),
            SensorClass::Power => Some(constants::HA_DEVICE_CLASS_SENSOR_POWER),
            SensorClass::Precipitation => Some(constants::HA_DEVICE_CLASS_SENSOR_PRECIPITATION),
            SensorClass::PrecipitationIntensity => Some(constants::HA_DEVICE_CLASS_SENSOR_PRECIPITATION_INTENSITY),
            SensorClass::Pressure => Some(constants::HA_DEVICE_CLASS_SENSOR_PRESSURE),
            SensorClass::ReactivePower => Some(constants::HA_DEVICE_CLASS_SENSOR_REACTIVE_POWER),
            SensorClass::SignalStrength => Some(constants::HA_DEVICE_CLASS_SENSOR_SIGNAL_STRENGTH),
            SensorClass::SoundPressure => Some(constants::HA_DEVICE_CLASS_SENSOR_SOUND_PRESSURE),
            SensorClass::Speed => Some(constants::HA_DEVICE_CLASS_SENSOR_SPEED),
            SensorClass::SulphurDioxide => Some(constants::HA_DEVICE_CLASS_SENSOR_SULPHUR_DIOXIDE),
            SensorClass::Temperature => Some(constants::HA_DEVICE_CLASS_SENSOR_TEMPERATURE),
            SensorClass::Timestamp => Some(constants::HA_DEVICE_CLASS_SENSOR_TIMESTAMP),
            SensorClass::VolatileOrganicCompounds => Some(constants::HA_DEVICE_CLASS_SENSOR_VOLATILE_ORGANIC_COMPOUNDS),
            SensorClass::VolatileOrganicCompoundsParts => Some(constants::HA_DEVICE_CLASS_SENSOR_VOLATILE_ORGANIC_COMPOUNDS_PARTS),
            SensorClass::Voltage => Some(constants::HA_DEVICE_CLASS_SENSOR_VOLTAGE),
            SensorClass::Volume => Some(constants::HA_DEVICE_CLASS_SENSOR_VOLUME),
            SensorClass::VolumeFlowRate => Some(constants::HA_DEVICE_CLASS_SENSOR_VOLUME_FLOW_RATE),
            SensorClass::VolumeStorage => Some(constants::HA_DEVICE_CLASS_SENSOR_VOLUME_STORAGE),
            SensorClass::Water => Some(constants::HA_DEVICE_CLASS_SENSOR_WATER),
            SensorClass::Weight => Some(constants::HA_DEVICE_CLASS_SENSOR_WEIGHT),
            SensorClass::WindSpeed => Some(constants::HA_DEVICE_CLASS_SENSOR_WIND_SPEED),
        }
    }
}

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
