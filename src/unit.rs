#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    #[default]
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

impl HumidityUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            HumidityUnit::Percentage => crate::constants::HA_UNIT_PERCENTAGE,
            HumidityUnit::Other(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryUnit {
    Percentage,
}

impl BatteryUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            BatteryUnit::Percentage => crate::constants::HA_UNIT_PERCENTAGE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightUnit {
    Lux,
}

impl LightUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            LightUnit::Lux => crate::constants::HA_UNIT_LIGHT_LUX,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureUnit {
    HectoPascal,
}

impl PressureUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            PressureUnit::HectoPascal => crate::constants::HA_UNIT_PRESSURE_HPA,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnergyUnit {
    KiloWattHour,
}

impl EnergyUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnergyUnit::KiloWattHour => crate::constants::HA_UNIT_ENERGY_KWH,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl TimeUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeUnit::Milliseconds => crate::constants::HA_UNIT_TIME_MILLISECONDS,
            TimeUnit::Seconds => crate::constants::HA_UNIT_TIME_SECONDS,
            TimeUnit::Minutes => crate::constants::HA_UNIT_TIME_MINUTES,
            TimeUnit::Hours => crate::constants::HA_UNIT_TIME_HOURS,
            TimeUnit::Days => crate::constants::HA_UNIT_TIME_DAYS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUnit {
    Watt,
    KiloWatt,
}

impl PowerUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            PowerUnit::Watt => crate::constants::HA_UNIT_POWER_WATT,
            PowerUnit::KiloWatt => crate::constants::HA_UNIT_POWER_KILOWATT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoltageUnit {
    Volt,
}

impl VoltageUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            VoltageUnit::Volt => crate::constants::HA_UNIT_VOLTAGE_VOLT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentUnit {
    Ampere,
}

impl CurrentUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            CurrentUnit::Ampere => crate::constants::HA_UNIT_CURRENT_AMPERE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceUnit {
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
}

impl DistanceUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            DistanceUnit::Millimeter => crate::constants::HA_UNIT_DISTANCE_MILLIMETER,
            DistanceUnit::Centimeter => crate::constants::HA_UNIT_DISTANCE_CENTIMETER,
            DistanceUnit::Meter => crate::constants::HA_UNIT_DISTANCE_METER,
            DistanceUnit::Kilometer => crate::constants::HA_UNIT_DISTANCE_KILOMETER,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrencyUnit {
    USD,
    EUR,
    GBP,
    JPY,
    CNY,
    CAD,
    AUD,
    CHF,
    INR,
    BRL,
    Dollar,
    Euro,
    Pound,
    Yen,
    Cent,
    Other(&'static str),
}

impl CurrencyUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            CurrencyUnit::USD => crate::constants::HA_UNIT_CURRENCY_USD,
            CurrencyUnit::EUR => crate::constants::HA_UNIT_CURRENCY_EUR,
            CurrencyUnit::GBP => crate::constants::HA_UNIT_CURRENCY_GBP,
            CurrencyUnit::JPY => crate::constants::HA_UNIT_CURRENCY_JPY,
            CurrencyUnit::CNY => crate::constants::HA_UNIT_CURRENCY_CNY,
            CurrencyUnit::CAD => crate::constants::HA_UNIT_CURRENCY_CAD,
            CurrencyUnit::AUD => crate::constants::HA_UNIT_CURRENCY_AUD,
            CurrencyUnit::CHF => crate::constants::HA_UNIT_CURRENCY_CHF,
            CurrencyUnit::INR => crate::constants::HA_UNIT_CURRENCY_INR,
            CurrencyUnit::BRL => crate::constants::HA_UNIT_CURRENCY_BRL,
            CurrencyUnit::Dollar => crate::constants::HA_UNIT_CURRENCY_DOLLAR,
            CurrencyUnit::Euro => crate::constants::HA_UNIT_CURRENCY_EURO,
            CurrencyUnit::Pound => crate::constants::HA_UNIT_CURRENCY_POUND,
            CurrencyUnit::Yen => crate::constants::HA_UNIT_CURRENCY_YEN,
            CurrencyUnit::Cent => crate::constants::HA_UNIT_CURRENCY_CENT,
            CurrencyUnit::Other(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberUnit {
    // Energy
    Joule,
    KiloJoule,
    MegaJoule,
    GigaJoule,
    MilliWattHour,
    WattHour,
    KiloWattHour,
    MegaWattHour,
    GigaWattHour,
    TeraWattHour,
    Calorie,
    KiloCalorie,
    MegaCalorie,
    GigaCalorie,
    // Temperature (°C, °F, K)
    Celsius,
    Fahrenheit,
    Kelvin,
    // Pressure
    MilliPascal,
    Pascal,
    HectoPascal,
    KiloPascal,
    Bar,
    CentiBar,
    MilliBar,
    MillimeterMercury,
    InchMercury,
    InchWater,
    Psi,
    // Volume
    Liter,
    MilliLiter,
    Gallon,
    FluidOunce,
    CubicMeter,
    CubicFoot,
    CCF,
    MCF,
    // Speed
    FeetPerSecond,
    InchPerDay,
    InchPerHour,
    InchPerSecond,
    KilometerPerHour,
    Knot,
    MeterPerSecond,
    MilePerHour,
    MillimeterPerDay,
    MillimeterPerSecond,
    // Distance
    Kilometer,
    Meter,
    Centimeter,
    Millimeter,
    Mile,
    NauticalMile,
    Yard,
    Inch,
    // Power
    MilliWatt,
    Watt,
    KiloWatt,
    MegaWatt,
    GigaWatt,
    TeraWatt,
    // Current
    Ampere,
    MilliAmpere,
    // Voltage
    Volt,
    MilliVolt,
    MicroVolt,
    KiloVolt,
    MegaVolt,
    // Data Rate
    BitPerSecond,
    KiloBitPerSecond,
    MegaBitPerSecond,
    GigaBitPerSecond,
    BytePerSecond,
    KiloBytePerSecond,
    MegaBytePerSecond,
    GigaBytePerSecond,
    KibiBytePerSecond,
    MebiBytePerSecond,
    GibiBytePerSecond,
    // Weight
    Kilogram,
    Gram,
    Milligram,
    Microgram,
    Ounce,
    Pound,
    Stone,
    // Other
    Percentage,
    Other(&'static str),
}

impl NumberUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            // Energy
            NumberUnit::Joule => crate::constants::HA_UNIT_ENERGY_JOULE,
            NumberUnit::KiloJoule => crate::constants::HA_UNIT_ENERGY_KILOJOULE,
            NumberUnit::MegaJoule => crate::constants::HA_UNIT_ENERGY_MEGAJOULE,
            NumberUnit::GigaJoule => crate::constants::HA_UNIT_ENERGY_GIGAJOULE,
            NumberUnit::MilliWattHour => crate::constants::HA_UNIT_ENERGY_MILLIWATTHOUR,
            NumberUnit::WattHour => crate::constants::HA_UNIT_ENERGY_WATTHOUR,
            NumberUnit::KiloWattHour => crate::constants::HA_UNIT_ENERGY_KWH,
            NumberUnit::MegaWattHour => crate::constants::HA_UNIT_ENERGY_MEGAWATTHOUR,
            NumberUnit::GigaWattHour => crate::constants::HA_UNIT_ENERGY_GIGAWATTHOUR,
            NumberUnit::TeraWattHour => crate::constants::HA_UNIT_ENERGY_TERAWATTHOUR,
            NumberUnit::Calorie => crate::constants::HA_UNIT_ENERGY_CALORIE,
            NumberUnit::KiloCalorie => crate::constants::HA_UNIT_ENERGY_KILOCALORIE,
            NumberUnit::MegaCalorie => crate::constants::HA_UNIT_ENERGY_MEGACALORIE,
            NumberUnit::GigaCalorie => crate::constants::HA_UNIT_ENERGY_GIGACALORIE,
            // Temperature
            NumberUnit::Celsius => crate::constants::HA_UNIT_TEMPERATURE_CELSIUS,
            NumberUnit::Fahrenheit => crate::constants::HA_UNIT_TEMPERATURE_FAHRENHEIT,
            NumberUnit::Kelvin => crate::constants::HA_UNIT_TEMPERATURE_KELVIN,
            // Pressure
            NumberUnit::MilliPascal => crate::constants::HA_UNIT_PRESSURE_MILLIPASCAL,
            NumberUnit::Pascal => crate::constants::HA_UNIT_PRESSURE_PASCAL,
            NumberUnit::HectoPascal => crate::constants::HA_UNIT_PRESSURE_HPA,
            NumberUnit::KiloPascal => crate::constants::HA_UNIT_PRESSURE_KILOPASCAL,
            NumberUnit::Bar => crate::constants::HA_UNIT_PRESSURE_BAR,
            NumberUnit::CentiBar => crate::constants::HA_UNIT_PRESSURE_CENTIBAR,
            NumberUnit::MilliBar => crate::constants::HA_UNIT_PRESSURE_MILLIBAR,
            NumberUnit::MillimeterMercury => crate::constants::HA_UNIT_PRESSURE_MILLIMETER_MERCURY,
            NumberUnit::InchMercury => crate::constants::HA_UNIT_PRESSURE_INCH_MERCURY,
            NumberUnit::InchWater => crate::constants::HA_UNIT_PRESSURE_INCH_WATER,
            NumberUnit::Psi => crate::constants::HA_UNIT_PRESSURE_PSI,
            // Volume
            NumberUnit::Liter => crate::constants::HA_UNIT_VOLUME_LITER,
            NumberUnit::MilliLiter => crate::constants::HA_UNIT_VOLUME_MILLILITER,
            NumberUnit::Gallon => crate::constants::HA_UNIT_VOLUME_GALLON,
            NumberUnit::FluidOunce => crate::constants::HA_UNIT_VOLUME_FLUID_OUNCE,
            NumberUnit::CubicMeter => crate::constants::HA_UNIT_VOLUME_CUBIC_METER,
            NumberUnit::CubicFoot => crate::constants::HA_UNIT_VOLUME_CUBIC_FOOT,
            NumberUnit::CCF => crate::constants::HA_UNIT_VOLUME_CCF,
            NumberUnit::MCF => crate::constants::HA_UNIT_VOLUME_MCF,
            // Speed
            NumberUnit::FeetPerSecond => crate::constants::HA_UNIT_SPEED_FEET_PER_SECOND,
            NumberUnit::InchPerDay => crate::constants::HA_UNIT_SPEED_INCH_PER_DAY,
            NumberUnit::InchPerHour => crate::constants::HA_UNIT_SPEED_INCH_PER_HOUR,
            NumberUnit::InchPerSecond => crate::constants::HA_UNIT_SPEED_INCH_PER_SECOND,
            NumberUnit::KilometerPerHour => crate::constants::HA_UNIT_SPEED_KILOMETER_PER_HOUR,
            NumberUnit::Knot => crate::constants::HA_UNIT_SPEED_KNOT,
            NumberUnit::MeterPerSecond => crate::constants::HA_UNIT_SPEED_METER_PER_SECOND,
            NumberUnit::MilePerHour => crate::constants::HA_UNIT_SPEED_MILE_PER_HOUR,
            NumberUnit::MillimeterPerDay => crate::constants::HA_UNIT_SPEED_MILLIMETER_PER_DAY,
            NumberUnit::MillimeterPerSecond => crate::constants::HA_UNIT_SPEED_MILLIMETER_PER_SECOND,
            // Distance
            NumberUnit::Kilometer => crate::constants::HA_UNIT_DISTANCE_KILOMETER,
            NumberUnit::Meter => crate::constants::HA_UNIT_DISTANCE_METER,
            NumberUnit::Centimeter => crate::constants::HA_UNIT_DISTANCE_CENTIMETER,
            NumberUnit::Millimeter => crate::constants::HA_UNIT_DISTANCE_MILLIMETER,
            NumberUnit::Mile => crate::constants::HA_UNIT_DISTANCE_MILE,
            NumberUnit::NauticalMile => crate::constants::HA_UNIT_DISTANCE_NAUTICAL_MILE,
            NumberUnit::Yard => crate::constants::HA_UNIT_DISTANCE_YARD,
            NumberUnit::Inch => crate::constants::HA_UNIT_DISTANCE_INCH,
            // Power
            NumberUnit::MilliWatt => crate::constants::HA_UNIT_POWER_MILLIWATT,
            NumberUnit::Watt => crate::constants::HA_UNIT_POWER_WATT,
            NumberUnit::KiloWatt => crate::constants::HA_UNIT_POWER_KILOWATT,
            NumberUnit::MegaWatt => crate::constants::HA_UNIT_POWER_MEGAWATT,
            NumberUnit::GigaWatt => crate::constants::HA_UNIT_POWER_GIGAWATT,
            NumberUnit::TeraWatt => crate::constants::HA_UNIT_POWER_TERAWATT,
            // Current
            NumberUnit::Ampere => crate::constants::HA_UNIT_CURRENT_AMPERE,
            NumberUnit::MilliAmpere => crate::constants::HA_UNIT_CURRENT_MILLIAMPERE,
            // Voltage
            NumberUnit::Volt => crate::constants::HA_UNIT_VOLTAGE_VOLT,
            NumberUnit::MilliVolt => crate::constants::HA_UNIT_VOLTAGE_MILLIVOLT,
            NumberUnit::MicroVolt => crate::constants::HA_UNIT_VOLTAGE_MICROVOLT,
            NumberUnit::KiloVolt => crate::constants::HA_UNIT_VOLTAGE_KILOVOLT,
            NumberUnit::MegaVolt => crate::constants::HA_UNIT_VOLTAGE_MEGAVOLT,
            // Data Rate
            NumberUnit::BitPerSecond => crate::constants::HA_UNIT_DATA_RATE_BIT_PER_SECOND,
            NumberUnit::KiloBitPerSecond => crate::constants::HA_UNIT_DATA_RATE_KILOBIT_PER_SECOND,
            NumberUnit::MegaBitPerSecond => crate::constants::HA_UNIT_DATA_RATE_MEGABIT_PER_SECOND,
            NumberUnit::GigaBitPerSecond => crate::constants::HA_UNIT_DATA_RATE_GIGABIT_PER_SECOND,
            NumberUnit::BytePerSecond => crate::constants::HA_UNIT_DATA_RATE_BYTE_PER_SECOND,
            NumberUnit::KiloBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_KILOBYTE_PER_SECOND,
            NumberUnit::MegaBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_MEGABYTE_PER_SECOND,
            NumberUnit::GigaBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_GIGABYTE_PER_SECOND,
            NumberUnit::KibiBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_KIBIBYTE_PER_SECOND,
            NumberUnit::MebiBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_MEBIBYTE_PER_SECOND,
            NumberUnit::GibiBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_GIBIBYTE_PER_SECOND,
            // Weight
            NumberUnit::Kilogram => crate::constants::HA_UNIT_WEIGHT_KILOGRAM,
            NumberUnit::Gram => crate::constants::HA_UNIT_WEIGHT_GRAM,
            NumberUnit::Milligram => crate::constants::HA_UNIT_WEIGHT_MILLIGRAM,
            NumberUnit::Microgram => crate::constants::HA_UNIT_WEIGHT_MICROGRAM,
            NumberUnit::Ounce => crate::constants::HA_UNIT_WEIGHT_OUNCE,
            NumberUnit::Pound => crate::constants::HA_UNIT_WEIGHT_POUND,
            NumberUnit::Stone => crate::constants::HA_UNIT_WEIGHT_STONE,
            // Other
            NumberUnit::Percentage => crate::constants::HA_UNIT_PERCENTAGE,
            NumberUnit::Other(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    TemperatureCelcius,
    TemperatureKelvin,
    TemperatureFahrenheit,
    HumidityPercentage,
    BatteryPercentage,
    LightLux,
    PressureHectoPascal,
    EnergyKiloWattHour,
    TimeMilliseconds,
    TimeSeconds,
    TimeMinutes,
    TimeHours,
    TimeDays,
    PowerWatt,
    PowerKiloWatt,
    VoltageVolt,
    CurrentAmpere,
    DistanceMillimeter,
    DistanceCentimeter,
    DistanceMeter,
    DistanceKilometer,
    CurrencyUSD,
    CurrencyEUR,
    CurrencyGBP,
    CurrencyJPY,
    CurrencyCNY,
    CurrencyCAD,
    CurrencyAUD,
    CurrencyCHF,
    CurrencyINR,
    CurrencyBRL,
    CurrencyDollar,
    CurrencyEuro,
    CurrencyPound,
    CurrencyYen,
    CurrencyCent,
    // Number units
    NumberJoule,
    NumberKiloJoule,
    NumberMegaJoule,
    NumberGigaJoule,
    NumberMilliWattHour,
    NumberWattHour,
    NumberKiloWattHour,
    NumberMegaWattHour,
    NumberGigaWattHour,
    NumberTeraWattHour,
    NumberCalorie,
    NumberKiloCalorie,
    NumberMegaCalorie,
    NumberGigaCalorie,
    NumberCelsius,
    NumberFahrenheit,
    NumberKelvin,
    NumberMilliPascal,
    NumberPascal,
    NumberHectoPascal,
    NumberKiloPascal,
    NumberBar,
    NumberCentiBar,
    NumberMilliBar,
    NumberMillimeterMercury,
    NumberInchMercury,
    NumberInchWater,
    NumberPsi,
    NumberLiter,
    NumberMilliLiter,
    NumberGallon,
    NumberFluidOunce,
    NumberCubicMeter,
    NumberCubicFoot,
    NumberCCF,
    NumberMCF,
    NumberFeetPerSecond,
    NumberInchPerDay,
    NumberInchPerHour,
    NumberInchPerSecond,
    NumberKilometerPerHour,
    NumberKnot,
    NumberMeterPerSecond,
    NumberMilePerHour,
    NumberMillimeterPerDay,
    NumberMillimeterPerSecond,
    NumberKilometer,
    NumberMeter,
    NumberCentimeter,
    NumberMillimeter,
    NumberMile,
    NumberNauticalMile,
    NumberYard,
    NumberInch,
    NumberMilliWatt,
    NumberWatt,
    NumberKiloWatt,
    NumberMegaWatt,
    NumberGigaWatt,
    NumberTeraWatt,
    NumberAmpere,
    NumberMilliAmpere,
    NumberVolt,
    NumberMilliVolt,
    NumberMicroVolt,
    NumberKiloVolt,
    NumberMegaVolt,
    NumberBitPerSecond,
    NumberKiloBitPerSecond,
    NumberMegaBitPerSecond,
    NumberGigaBitPerSecond,
    NumberBytePerSecond,
    NumberKiloBytePerSecond,
    NumberMegaBytePerSecond,
    NumberGigaBytePerSecond,
    NumberKibiBytePerSecond,
    NumberMebiBytePerSecond,
    NumberGibiBytePerSecond,
    NumberKilogram,
    NumberGram,
    NumberMilligram,
    NumberMicrogram,
    NumberOunce,
    NumberPound,
    NumberStone,
    NumberPercentage,
    Other(&'static str),
}

impl Unit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Unit::TemperatureCelcius => crate::constants::HA_UNIT_TEMPERATURE_CELSIUS,
            Unit::TemperatureKelvin => crate::constants::HA_UNIT_TEMPERATURE_KELVIN,
            Unit::TemperatureFahrenheit => crate::constants::HA_UNIT_TEMPERATURE_FAHRENHEIT,
            Unit::HumidityPercentage => crate::constants::HA_UNIT_PERCENTAGE,
            Unit::BatteryPercentage => crate::constants::HA_UNIT_PERCENTAGE,
            Unit::LightLux => crate::constants::HA_UNIT_LIGHT_LUX,
            Unit::PressureHectoPascal => crate::constants::HA_UNIT_PRESSURE_HPA,
            Unit::EnergyKiloWattHour => crate::constants::HA_UNIT_ENERGY_KWH,
            Unit::TimeMilliseconds => crate::constants::HA_UNIT_TIME_MILLISECONDS,
            Unit::TimeSeconds => crate::constants::HA_UNIT_TIME_SECONDS,
            Unit::TimeMinutes => crate::constants::HA_UNIT_TIME_MINUTES,
            Unit::TimeHours => crate::constants::HA_UNIT_TIME_HOURS,
            Unit::TimeDays => crate::constants::HA_UNIT_TIME_DAYS,
            Unit::PowerWatt => crate::constants::HA_UNIT_POWER_WATT,
            Unit::PowerKiloWatt => crate::constants::HA_UNIT_POWER_KILOWATT,
            Unit::VoltageVolt => crate::constants::HA_UNIT_VOLTAGE_VOLT,
            Unit::CurrentAmpere => crate::constants::HA_UNIT_CURRENT_AMPERE,
            Unit::DistanceMillimeter => crate::constants::HA_UNIT_DISTANCE_MILLIMETER,
            Unit::DistanceCentimeter => crate::constants::HA_UNIT_DISTANCE_CENTIMETER,
            Unit::DistanceMeter => crate::constants::HA_UNIT_DISTANCE_METER,
            Unit::DistanceKilometer => crate::constants::HA_UNIT_DISTANCE_KILOMETER,
            Unit::CurrencyUSD => crate::constants::HA_UNIT_CURRENCY_USD,
            Unit::CurrencyEUR => crate::constants::HA_UNIT_CURRENCY_EUR,
            Unit::CurrencyGBP => crate::constants::HA_UNIT_CURRENCY_GBP,
            Unit::CurrencyJPY => crate::constants::HA_UNIT_CURRENCY_JPY,
            Unit::CurrencyCNY => crate::constants::HA_UNIT_CURRENCY_CNY,
            Unit::CurrencyCAD => crate::constants::HA_UNIT_CURRENCY_CAD,
            Unit::CurrencyAUD => crate::constants::HA_UNIT_CURRENCY_AUD,
            Unit::CurrencyCHF => crate::constants::HA_UNIT_CURRENCY_CHF,
            Unit::CurrencyINR => crate::constants::HA_UNIT_CURRENCY_INR,
            Unit::CurrencyBRL => crate::constants::HA_UNIT_CURRENCY_BRL,
            Unit::CurrencyDollar => crate::constants::HA_UNIT_CURRENCY_DOLLAR,
            Unit::CurrencyEuro => crate::constants::HA_UNIT_CURRENCY_EURO,
            Unit::CurrencyPound => crate::constants::HA_UNIT_CURRENCY_POUND,
            Unit::CurrencyYen => crate::constants::HA_UNIT_CURRENCY_YEN,
            Unit::CurrencyCent => crate::constants::HA_UNIT_CURRENCY_CENT,
            // Number units
            Unit::NumberJoule => crate::constants::HA_UNIT_ENERGY_JOULE,
            Unit::NumberKiloJoule => crate::constants::HA_UNIT_ENERGY_KILOJOULE,
            Unit::NumberMegaJoule => crate::constants::HA_UNIT_ENERGY_MEGAJOULE,
            Unit::NumberGigaJoule => crate::constants::HA_UNIT_ENERGY_GIGAJOULE,
            Unit::NumberMilliWattHour => crate::constants::HA_UNIT_ENERGY_MILLIWATTHOUR,
            Unit::NumberWattHour => crate::constants::HA_UNIT_ENERGY_WATTHOUR,
            Unit::NumberKiloWattHour => crate::constants::HA_UNIT_ENERGY_KWH,
            Unit::NumberMegaWattHour => crate::constants::HA_UNIT_ENERGY_MEGAWATTHOUR,
            Unit::NumberGigaWattHour => crate::constants::HA_UNIT_ENERGY_GIGAWATTHOUR,
            Unit::NumberTeraWattHour => crate::constants::HA_UNIT_ENERGY_TERAWATTHOUR,
            Unit::NumberCalorie => crate::constants::HA_UNIT_ENERGY_CALORIE,
            Unit::NumberKiloCalorie => crate::constants::HA_UNIT_ENERGY_KILOCALORIE,
            Unit::NumberMegaCalorie => crate::constants::HA_UNIT_ENERGY_MEGACALORIE,
            Unit::NumberGigaCalorie => crate::constants::HA_UNIT_ENERGY_GIGACALORIE,
            Unit::NumberCelsius => crate::constants::HA_UNIT_TEMPERATURE_CELSIUS,
            Unit::NumberFahrenheit => crate::constants::HA_UNIT_TEMPERATURE_FAHRENHEIT,
            Unit::NumberKelvin => crate::constants::HA_UNIT_TEMPERATURE_KELVIN,
            Unit::NumberMilliPascal => crate::constants::HA_UNIT_PRESSURE_MILLIPASCAL,
            Unit::NumberPascal => crate::constants::HA_UNIT_PRESSURE_PASCAL,
            Unit::NumberHectoPascal => crate::constants::HA_UNIT_PRESSURE_HPA,
            Unit::NumberKiloPascal => crate::constants::HA_UNIT_PRESSURE_KILOPASCAL,
            Unit::NumberBar => crate::constants::HA_UNIT_PRESSURE_BAR,
            Unit::NumberCentiBar => crate::constants::HA_UNIT_PRESSURE_CENTIBAR,
            Unit::NumberMilliBar => crate::constants::HA_UNIT_PRESSURE_MILLIBAR,
            Unit::NumberMillimeterMercury => crate::constants::HA_UNIT_PRESSURE_MILLIMETER_MERCURY,
            Unit::NumberInchMercury => crate::constants::HA_UNIT_PRESSURE_INCH_MERCURY,
            Unit::NumberInchWater => crate::constants::HA_UNIT_PRESSURE_INCH_WATER,
            Unit::NumberPsi => crate::constants::HA_UNIT_PRESSURE_PSI,
            Unit::NumberLiter => crate::constants::HA_UNIT_VOLUME_LITER,
            Unit::NumberMilliLiter => crate::constants::HA_UNIT_VOLUME_MILLILITER,
            Unit::NumberGallon => crate::constants::HA_UNIT_VOLUME_GALLON,
            Unit::NumberFluidOunce => crate::constants::HA_UNIT_VOLUME_FLUID_OUNCE,
            Unit::NumberCubicMeter => crate::constants::HA_UNIT_VOLUME_CUBIC_METER,
            Unit::NumberCubicFoot => crate::constants::HA_UNIT_VOLUME_CUBIC_FOOT,
            Unit::NumberCCF => crate::constants::HA_UNIT_VOLUME_CCF,
            Unit::NumberMCF => crate::constants::HA_UNIT_VOLUME_MCF,
            Unit::NumberFeetPerSecond => crate::constants::HA_UNIT_SPEED_FEET_PER_SECOND,
            Unit::NumberInchPerDay => crate::constants::HA_UNIT_SPEED_INCH_PER_DAY,
            Unit::NumberInchPerHour => crate::constants::HA_UNIT_SPEED_INCH_PER_HOUR,
            Unit::NumberInchPerSecond => crate::constants::HA_UNIT_SPEED_INCH_PER_SECOND,
            Unit::NumberKilometerPerHour => crate::constants::HA_UNIT_SPEED_KILOMETER_PER_HOUR,
            Unit::NumberKnot => crate::constants::HA_UNIT_SPEED_KNOT,
            Unit::NumberMeterPerSecond => crate::constants::HA_UNIT_SPEED_METER_PER_SECOND,
            Unit::NumberMilePerHour => crate::constants::HA_UNIT_SPEED_MILE_PER_HOUR,
            Unit::NumberMillimeterPerDay => crate::constants::HA_UNIT_SPEED_MILLIMETER_PER_DAY,
            Unit::NumberMillimeterPerSecond => crate::constants::HA_UNIT_SPEED_MILLIMETER_PER_SECOND,
            Unit::NumberKilometer => crate::constants::HA_UNIT_DISTANCE_KILOMETER,
            Unit::NumberMeter => crate::constants::HA_UNIT_DISTANCE_METER,
            Unit::NumberCentimeter => crate::constants::HA_UNIT_DISTANCE_CENTIMETER,
            Unit::NumberMillimeter => crate::constants::HA_UNIT_DISTANCE_MILLIMETER,
            Unit::NumberMile => crate::constants::HA_UNIT_DISTANCE_MILE,
            Unit::NumberNauticalMile => crate::constants::HA_UNIT_DISTANCE_NAUTICAL_MILE,
            Unit::NumberYard => crate::constants::HA_UNIT_DISTANCE_YARD,
            Unit::NumberInch => crate::constants::HA_UNIT_DISTANCE_INCH,
            Unit::NumberMilliWatt => crate::constants::HA_UNIT_POWER_MILLIWATT,
            Unit::NumberWatt => crate::constants::HA_UNIT_POWER_WATT,
            Unit::NumberKiloWatt => crate::constants::HA_UNIT_POWER_KILOWATT,
            Unit::NumberMegaWatt => crate::constants::HA_UNIT_POWER_MEGAWATT,
            Unit::NumberGigaWatt => crate::constants::HA_UNIT_POWER_GIGAWATT,
            Unit::NumberTeraWatt => crate::constants::HA_UNIT_POWER_TERAWATT,
            Unit::NumberAmpere => crate::constants::HA_UNIT_CURRENT_AMPERE,
            Unit::NumberMilliAmpere => crate::constants::HA_UNIT_CURRENT_MILLIAMPERE,
            Unit::NumberVolt => crate::constants::HA_UNIT_VOLTAGE_VOLT,
            Unit::NumberMilliVolt => crate::constants::HA_UNIT_VOLTAGE_MILLIVOLT,
            Unit::NumberMicroVolt => crate::constants::HA_UNIT_VOLTAGE_MICROVOLT,
            Unit::NumberKiloVolt => crate::constants::HA_UNIT_VOLTAGE_KILOVOLT,
            Unit::NumberMegaVolt => crate::constants::HA_UNIT_VOLTAGE_MEGAVOLT,
            Unit::NumberBitPerSecond => crate::constants::HA_UNIT_DATA_RATE_BIT_PER_SECOND,
            Unit::NumberKiloBitPerSecond => crate::constants::HA_UNIT_DATA_RATE_KILOBIT_PER_SECOND,
            Unit::NumberMegaBitPerSecond => crate::constants::HA_UNIT_DATA_RATE_MEGABIT_PER_SECOND,
            Unit::NumberGigaBitPerSecond => crate::constants::HA_UNIT_DATA_RATE_GIGABIT_PER_SECOND,
            Unit::NumberBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_BYTE_PER_SECOND,
            Unit::NumberKiloBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_KILOBYTE_PER_SECOND,
            Unit::NumberMegaBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_MEGABYTE_PER_SECOND,
            Unit::NumberGigaBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_GIGABYTE_PER_SECOND,
            Unit::NumberKibiBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_KIBIBYTE_PER_SECOND,
            Unit::NumberMebiBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_MEBIBYTE_PER_SECOND,
            Unit::NumberGibiBytePerSecond => crate::constants::HA_UNIT_DATA_RATE_GIBIBYTE_PER_SECOND,
            Unit::NumberKilogram => crate::constants::HA_UNIT_WEIGHT_KILOGRAM,
            Unit::NumberGram => crate::constants::HA_UNIT_WEIGHT_GRAM,
            Unit::NumberMilligram => crate::constants::HA_UNIT_WEIGHT_MILLIGRAM,
            Unit::NumberMicrogram => crate::constants::HA_UNIT_WEIGHT_MICROGRAM,
            Unit::NumberOunce => crate::constants::HA_UNIT_WEIGHT_OUNCE,
            Unit::NumberPound => crate::constants::HA_UNIT_WEIGHT_POUND,
            Unit::NumberStone => crate::constants::HA_UNIT_WEIGHT_STONE,
            Unit::NumberPercentage => crate::constants::HA_UNIT_PERCENTAGE,
            Unit::Other(other) => other,
        }
    }
}

// From conversions
impl From<TemperatureUnit> for Unit {
    fn from(unit: TemperatureUnit) -> Self {
        match unit {
            TemperatureUnit::Celcius => Unit::TemperatureCelcius,
            TemperatureUnit::Kelvin => Unit::TemperatureKelvin,
            TemperatureUnit::Fahrenheit => Unit::TemperatureFahrenheit,
            TemperatureUnit::Other(other) => Unit::Other(other),
        }
    }
}

impl From<HumidityUnit> for Unit {
    fn from(unit: HumidityUnit) -> Self {
        match unit {
            HumidityUnit::Percentage => Unit::HumidityPercentage,
            HumidityUnit::Other(other) => Unit::Other(other),
        }
    }
}

impl From<BatteryUnit> for Unit {
    fn from(unit: BatteryUnit) -> Self {
        match unit {
            BatteryUnit::Percentage => Unit::BatteryPercentage,
        }
    }
}

impl From<LightUnit> for Unit {
    fn from(unit: LightUnit) -> Self {
        match unit {
            LightUnit::Lux => Unit::LightLux,
        }
    }
}

impl From<PressureUnit> for Unit {
    fn from(unit: PressureUnit) -> Self {
        match unit {
            PressureUnit::HectoPascal => Unit::PressureHectoPascal,
        }
    }
}

impl From<EnergyUnit> for Unit {
    fn from(unit: EnergyUnit) -> Self {
        match unit {
            EnergyUnit::KiloWattHour => Unit::EnergyKiloWattHour,
        }
    }
}

impl From<TimeUnit> for Unit {
    fn from(unit: TimeUnit) -> Self {
        match unit {
            TimeUnit::Milliseconds => Unit::TimeMilliseconds,
            TimeUnit::Seconds => Unit::TimeSeconds,
            TimeUnit::Minutes => Unit::TimeMinutes,
            TimeUnit::Hours => Unit::TimeHours,
            TimeUnit::Days => Unit::TimeDays,
        }
    }
}

impl From<PowerUnit> for Unit {
    fn from(unit: PowerUnit) -> Self {
        match unit {
            PowerUnit::Watt => Unit::PowerWatt,
            PowerUnit::KiloWatt => Unit::PowerKiloWatt,
        }
    }
}

impl From<VoltageUnit> for Unit {
    fn from(unit: VoltageUnit) -> Self {
        match unit {
            VoltageUnit::Volt => Unit::VoltageVolt,
        }
    }
}

impl From<CurrentUnit> for Unit {
    fn from(unit: CurrentUnit) -> Self {
        match unit {
            CurrentUnit::Ampere => Unit::CurrentAmpere,
        }
    }
}

impl From<DistanceUnit> for Unit {
    fn from(unit: DistanceUnit) -> Self {
        match unit {
            DistanceUnit::Millimeter => Unit::DistanceMillimeter,
            DistanceUnit::Centimeter => Unit::DistanceCentimeter,
            DistanceUnit::Meter => Unit::DistanceMeter,
            DistanceUnit::Kilometer => Unit::DistanceKilometer,
        }
    }
}

impl From<CurrencyUnit> for Unit {
    fn from(unit: CurrencyUnit) -> Self {
        match unit {
            CurrencyUnit::USD => Unit::CurrencyUSD,
            CurrencyUnit::EUR => Unit::CurrencyEUR,
            CurrencyUnit::GBP => Unit::CurrencyGBP,
            CurrencyUnit::JPY => Unit::CurrencyJPY,
            CurrencyUnit::CNY => Unit::CurrencyCNY,
            CurrencyUnit::CAD => Unit::CurrencyCAD,
            CurrencyUnit::AUD => Unit::CurrencyAUD,
            CurrencyUnit::CHF => Unit::CurrencyCHF,
            CurrencyUnit::INR => Unit::CurrencyINR,
            CurrencyUnit::BRL => Unit::CurrencyBRL,
            CurrencyUnit::Dollar => Unit::CurrencyDollar,
            CurrencyUnit::Euro => Unit::CurrencyEuro,
            CurrencyUnit::Pound => Unit::CurrencyPound,
            CurrencyUnit::Yen => Unit::CurrencyYen,
            CurrencyUnit::Cent => Unit::CurrencyCent,
            CurrencyUnit::Other(other) => Unit::Other(other),
        }
    }
}

impl From<NumberUnit> for Unit {
    fn from(unit: NumberUnit) -> Self {
        match unit {
            NumberUnit::Joule => Unit::NumberJoule,
            NumberUnit::KiloJoule => Unit::NumberKiloJoule,
            NumberUnit::MegaJoule => Unit::NumberMegaJoule,
            NumberUnit::GigaJoule => Unit::NumberGigaJoule,
            NumberUnit::MilliWattHour => Unit::NumberMilliWattHour,
            NumberUnit::WattHour => Unit::NumberWattHour,
            NumberUnit::KiloWattHour => Unit::NumberKiloWattHour,
            NumberUnit::MegaWattHour => Unit::NumberMegaWattHour,
            NumberUnit::GigaWattHour => Unit::NumberGigaWattHour,
            NumberUnit::TeraWattHour => Unit::NumberTeraWattHour,
            NumberUnit::Calorie => Unit::NumberCalorie,
            NumberUnit::KiloCalorie => Unit::NumberKiloCalorie,
            NumberUnit::MegaCalorie => Unit::NumberMegaCalorie,
            NumberUnit::GigaCalorie => Unit::NumberGigaCalorie,
            NumberUnit::Celsius => Unit::NumberCelsius,
            NumberUnit::Fahrenheit => Unit::NumberFahrenheit,
            NumberUnit::Kelvin => Unit::NumberKelvin,
            NumberUnit::MilliPascal => Unit::NumberMilliPascal,
            NumberUnit::Pascal => Unit::NumberPascal,
            NumberUnit::HectoPascal => Unit::NumberHectoPascal,
            NumberUnit::KiloPascal => Unit::NumberKiloPascal,
            NumberUnit::Bar => Unit::NumberBar,
            NumberUnit::CentiBar => Unit::NumberCentiBar,
            NumberUnit::MilliBar => Unit::NumberMilliBar,
            NumberUnit::MillimeterMercury => Unit::NumberMillimeterMercury,
            NumberUnit::InchMercury => Unit::NumberInchMercury,
            NumberUnit::InchWater => Unit::NumberInchWater,
            NumberUnit::Psi => Unit::NumberPsi,
            NumberUnit::Liter => Unit::NumberLiter,
            NumberUnit::MilliLiter => Unit::NumberMilliLiter,
            NumberUnit::Gallon => Unit::NumberGallon,
            NumberUnit::FluidOunce => Unit::NumberFluidOunce,
            NumberUnit::CubicMeter => Unit::NumberCubicMeter,
            NumberUnit::CubicFoot => Unit::NumberCubicFoot,
            NumberUnit::CCF => Unit::NumberCCF,
            NumberUnit::MCF => Unit::NumberMCF,
            NumberUnit::FeetPerSecond => Unit::NumberFeetPerSecond,
            NumberUnit::InchPerDay => Unit::NumberInchPerDay,
            NumberUnit::InchPerHour => Unit::NumberInchPerHour,
            NumberUnit::InchPerSecond => Unit::NumberInchPerSecond,
            NumberUnit::KilometerPerHour => Unit::NumberKilometerPerHour,
            NumberUnit::Knot => Unit::NumberKnot,
            NumberUnit::MeterPerSecond => Unit::NumberMeterPerSecond,
            NumberUnit::MilePerHour => Unit::NumberMilePerHour,
            NumberUnit::MillimeterPerDay => Unit::NumberMillimeterPerDay,
            NumberUnit::MillimeterPerSecond => Unit::NumberMillimeterPerSecond,
            NumberUnit::Kilometer => Unit::NumberKilometer,
            NumberUnit::Meter => Unit::NumberMeter,
            NumberUnit::Centimeter => Unit::NumberCentimeter,
            NumberUnit::Millimeter => Unit::NumberMillimeter,
            NumberUnit::Mile => Unit::NumberMile,
            NumberUnit::NauticalMile => Unit::NumberNauticalMile,
            NumberUnit::Yard => Unit::NumberYard,
            NumberUnit::Inch => Unit::NumberInch,
            NumberUnit::MilliWatt => Unit::NumberMilliWatt,
            NumberUnit::Watt => Unit::NumberWatt,
            NumberUnit::KiloWatt => Unit::NumberKiloWatt,
            NumberUnit::MegaWatt => Unit::NumberMegaWatt,
            NumberUnit::GigaWatt => Unit::NumberGigaWatt,
            NumberUnit::TeraWatt => Unit::NumberTeraWatt,
            NumberUnit::Ampere => Unit::NumberAmpere,
            NumberUnit::MilliAmpere => Unit::NumberMilliAmpere,
            NumberUnit::Volt => Unit::NumberVolt,
            NumberUnit::MilliVolt => Unit::NumberMilliVolt,
            NumberUnit::MicroVolt => Unit::NumberMicroVolt,
            NumberUnit::KiloVolt => Unit::NumberKiloVolt,
            NumberUnit::MegaVolt => Unit::NumberMegaVolt,
            NumberUnit::BitPerSecond => Unit::NumberBitPerSecond,
            NumberUnit::KiloBitPerSecond => Unit::NumberKiloBitPerSecond,
            NumberUnit::MegaBitPerSecond => Unit::NumberMegaBitPerSecond,
            NumberUnit::GigaBitPerSecond => Unit::NumberGigaBitPerSecond,
            NumberUnit::BytePerSecond => Unit::NumberBytePerSecond,
            NumberUnit::KiloBytePerSecond => Unit::NumberKiloBytePerSecond,
            NumberUnit::MegaBytePerSecond => Unit::NumberMegaBytePerSecond,
            NumberUnit::GigaBytePerSecond => Unit::NumberGigaBytePerSecond,
            NumberUnit::KibiBytePerSecond => Unit::NumberKibiBytePerSecond,
            NumberUnit::MebiBytePerSecond => Unit::NumberMebiBytePerSecond,
            NumberUnit::GibiBytePerSecond => Unit::NumberGibiBytePerSecond,
            NumberUnit::Kilogram => Unit::NumberKilogram,
            NumberUnit::Gram => Unit::NumberGram,
            NumberUnit::Milligram => Unit::NumberMilligram,
            NumberUnit::Microgram => Unit::NumberMicrogram,
            NumberUnit::Ounce => Unit::NumberOunce,
            NumberUnit::Pound => Unit::NumberPound,
            NumberUnit::Stone => Unit::NumberStone,
            NumberUnit::Percentage => Unit::NumberPercentage,
            NumberUnit::Other(other) => Unit::Other(other),
        }
    }
}

// TryFrom conversions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitConversionError;

impl TryFrom<Unit> for TemperatureUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::TemperatureCelcius => Ok(TemperatureUnit::Celcius),
            Unit::TemperatureKelvin => Ok(TemperatureUnit::Kelvin),
            Unit::TemperatureFahrenheit => Ok(TemperatureUnit::Fahrenheit),
            Unit::Other(other) => Ok(TemperatureUnit::Other(other)),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for HumidityUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::HumidityPercentage => Ok(HumidityUnit::Percentage),
            Unit::Other(other) => Ok(HumidityUnit::Other(other)),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for BatteryUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::BatteryPercentage => Ok(BatteryUnit::Percentage),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for LightUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::LightLux => Ok(LightUnit::Lux),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for PressureUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::PressureHectoPascal => Ok(PressureUnit::HectoPascal),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for EnergyUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::EnergyKiloWattHour => Ok(EnergyUnit::KiloWattHour),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for TimeUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::TimeMilliseconds => Ok(TimeUnit::Milliseconds),
            Unit::TimeSeconds => Ok(TimeUnit::Seconds),
            Unit::TimeMinutes => Ok(TimeUnit::Minutes),
            Unit::TimeHours => Ok(TimeUnit::Hours),
            Unit::TimeDays => Ok(TimeUnit::Days),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for PowerUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::PowerWatt => Ok(PowerUnit::Watt),
            Unit::PowerKiloWatt => Ok(PowerUnit::KiloWatt),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for VoltageUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::VoltageVolt => Ok(VoltageUnit::Volt),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for CurrentUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::CurrentAmpere => Ok(CurrentUnit::Ampere),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for DistanceUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::DistanceMillimeter => Ok(DistanceUnit::Millimeter),
            Unit::DistanceCentimeter => Ok(DistanceUnit::Centimeter),
            Unit::DistanceMeter => Ok(DistanceUnit::Meter),
            Unit::DistanceKilometer => Ok(DistanceUnit::Kilometer),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for CurrencyUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::CurrencyUSD => Ok(CurrencyUnit::USD),
            Unit::CurrencyEUR => Ok(CurrencyUnit::EUR),
            Unit::CurrencyGBP => Ok(CurrencyUnit::GBP),
            Unit::CurrencyJPY => Ok(CurrencyUnit::JPY),
            Unit::CurrencyCNY => Ok(CurrencyUnit::CNY),
            Unit::CurrencyCAD => Ok(CurrencyUnit::CAD),
            Unit::CurrencyAUD => Ok(CurrencyUnit::AUD),
            Unit::CurrencyCHF => Ok(CurrencyUnit::CHF),
            Unit::CurrencyINR => Ok(CurrencyUnit::INR),
            Unit::CurrencyBRL => Ok(CurrencyUnit::BRL),
            Unit::CurrencyDollar => Ok(CurrencyUnit::Dollar),
            Unit::CurrencyEuro => Ok(CurrencyUnit::Euro),
            Unit::CurrencyPound => Ok(CurrencyUnit::Pound),
            Unit::CurrencyYen => Ok(CurrencyUnit::Yen),
            Unit::CurrencyCent => Ok(CurrencyUnit::Cent),
            Unit::Other(other) => Ok(CurrencyUnit::Other(other)),
            _ => Err(UnitConversionError),
        }
    }
}

impl TryFrom<Unit> for NumberUnit {
    type Error = UnitConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        match unit {
            Unit::NumberJoule => Ok(NumberUnit::Joule),
            Unit::NumberKiloJoule => Ok(NumberUnit::KiloJoule),
            Unit::NumberMegaJoule => Ok(NumberUnit::MegaJoule),
            Unit::NumberGigaJoule => Ok(NumberUnit::GigaJoule),
            Unit::NumberMilliWattHour => Ok(NumberUnit::MilliWattHour),
            Unit::NumberWattHour => Ok(NumberUnit::WattHour),
            Unit::NumberKiloWattHour => Ok(NumberUnit::KiloWattHour),
            Unit::NumberMegaWattHour => Ok(NumberUnit::MegaWattHour),
            Unit::NumberGigaWattHour => Ok(NumberUnit::GigaWattHour),
            Unit::NumberTeraWattHour => Ok(NumberUnit::TeraWattHour),
            Unit::NumberCalorie => Ok(NumberUnit::Calorie),
            Unit::NumberKiloCalorie => Ok(NumberUnit::KiloCalorie),
            Unit::NumberMegaCalorie => Ok(NumberUnit::MegaCalorie),
            Unit::NumberGigaCalorie => Ok(NumberUnit::GigaCalorie),
            Unit::NumberCelsius => Ok(NumberUnit::Celsius),
            Unit::NumberFahrenheit => Ok(NumberUnit::Fahrenheit),
            Unit::NumberKelvin => Ok(NumberUnit::Kelvin),
            Unit::NumberMilliPascal => Ok(NumberUnit::MilliPascal),
            Unit::NumberPascal => Ok(NumberUnit::Pascal),
            Unit::NumberHectoPascal => Ok(NumberUnit::HectoPascal),
            Unit::NumberKiloPascal => Ok(NumberUnit::KiloPascal),
            Unit::NumberBar => Ok(NumberUnit::Bar),
            Unit::NumberCentiBar => Ok(NumberUnit::CentiBar),
            Unit::NumberMilliBar => Ok(NumberUnit::MilliBar),
            Unit::NumberMillimeterMercury => Ok(NumberUnit::MillimeterMercury),
            Unit::NumberInchMercury => Ok(NumberUnit::InchMercury),
            Unit::NumberInchWater => Ok(NumberUnit::InchWater),
            Unit::NumberPsi => Ok(NumberUnit::Psi),
            Unit::NumberLiter => Ok(NumberUnit::Liter),
            Unit::NumberMilliLiter => Ok(NumberUnit::MilliLiter),
            Unit::NumberGallon => Ok(NumberUnit::Gallon),
            Unit::NumberFluidOunce => Ok(NumberUnit::FluidOunce),
            Unit::NumberCubicMeter => Ok(NumberUnit::CubicMeter),
            Unit::NumberCubicFoot => Ok(NumberUnit::CubicFoot),
            Unit::NumberCCF => Ok(NumberUnit::CCF),
            Unit::NumberMCF => Ok(NumberUnit::MCF),
            Unit::NumberFeetPerSecond => Ok(NumberUnit::FeetPerSecond),
            Unit::NumberInchPerDay => Ok(NumberUnit::InchPerDay),
            Unit::NumberInchPerHour => Ok(NumberUnit::InchPerHour),
            Unit::NumberInchPerSecond => Ok(NumberUnit::InchPerSecond),
            Unit::NumberKilometerPerHour => Ok(NumberUnit::KilometerPerHour),
            Unit::NumberKnot => Ok(NumberUnit::Knot),
            Unit::NumberMeterPerSecond => Ok(NumberUnit::MeterPerSecond),
            Unit::NumberMilePerHour => Ok(NumberUnit::MilePerHour),
            Unit::NumberMillimeterPerDay => Ok(NumberUnit::MillimeterPerDay),
            Unit::NumberMillimeterPerSecond => Ok(NumberUnit::MillimeterPerSecond),
            Unit::NumberKilometer => Ok(NumberUnit::Kilometer),
            Unit::NumberMeter => Ok(NumberUnit::Meter),
            Unit::NumberCentimeter => Ok(NumberUnit::Centimeter),
            Unit::NumberMillimeter => Ok(NumberUnit::Millimeter),
            Unit::NumberMile => Ok(NumberUnit::Mile),
            Unit::NumberNauticalMile => Ok(NumberUnit::NauticalMile),
            Unit::NumberYard => Ok(NumberUnit::Yard),
            Unit::NumberInch => Ok(NumberUnit::Inch),
            Unit::NumberMilliWatt => Ok(NumberUnit::MilliWatt),
            Unit::NumberWatt => Ok(NumberUnit::Watt),
            Unit::NumberKiloWatt => Ok(NumberUnit::KiloWatt),
            Unit::NumberMegaWatt => Ok(NumberUnit::MegaWatt),
            Unit::NumberGigaWatt => Ok(NumberUnit::GigaWatt),
            Unit::NumberTeraWatt => Ok(NumberUnit::TeraWatt),
            Unit::NumberAmpere => Ok(NumberUnit::Ampere),
            Unit::NumberMilliAmpere => Ok(NumberUnit::MilliAmpere),
            Unit::NumberVolt => Ok(NumberUnit::Volt),
            Unit::NumberMilliVolt => Ok(NumberUnit::MilliVolt),
            Unit::NumberMicroVolt => Ok(NumberUnit::MicroVolt),
            Unit::NumberKiloVolt => Ok(NumberUnit::KiloVolt),
            Unit::NumberMegaVolt => Ok(NumberUnit::MegaVolt),
            Unit::NumberBitPerSecond => Ok(NumberUnit::BitPerSecond),
            Unit::NumberKiloBitPerSecond => Ok(NumberUnit::KiloBitPerSecond),
            Unit::NumberMegaBitPerSecond => Ok(NumberUnit::MegaBitPerSecond),
            Unit::NumberGigaBitPerSecond => Ok(NumberUnit::GigaBitPerSecond),
            Unit::NumberBytePerSecond => Ok(NumberUnit::BytePerSecond),
            Unit::NumberKiloBytePerSecond => Ok(NumberUnit::KiloBytePerSecond),
            Unit::NumberMegaBytePerSecond => Ok(NumberUnit::MegaBytePerSecond),
            Unit::NumberGigaBytePerSecond => Ok(NumberUnit::GigaBytePerSecond),
            Unit::NumberKibiBytePerSecond => Ok(NumberUnit::KibiBytePerSecond),
            Unit::NumberMebiBytePerSecond => Ok(NumberUnit::MebiBytePerSecond),
            Unit::NumberGibiBytePerSecond => Ok(NumberUnit::GibiBytePerSecond),
            Unit::NumberKilogram => Ok(NumberUnit::Kilogram),
            Unit::NumberGram => Ok(NumberUnit::Gram),
            Unit::NumberMilligram => Ok(NumberUnit::Milligram),
            Unit::NumberMicrogram => Ok(NumberUnit::Microgram),
            Unit::NumberOunce => Ok(NumberUnit::Ounce),
            Unit::NumberPound => Ok(NumberUnit::Pound),
            Unit::NumberStone => Ok(NumberUnit::Stone),
            Unit::NumberPercentage => Ok(NumberUnit::Percentage),
            Unit::Other(other) => Ok(NumberUnit::Other(other)),
            _ => Err(UnitConversionError),
        }
    }
}
