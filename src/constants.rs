#![allow(unused)]

pub const HA_STATUS_TOPIC: &str = "homeassistant/status";
pub const HA_STATUS_PAYLOAD_ONLINE: &str = "online";
pub const HA_STATUS_PAYLOAD_OFFLINE: &str = "offline";

pub const HA_DOMAIN_SENSOR: &str = "sensor";
pub const HA_DOMAIN_BINARY_SENSOR: &str = "binary_sensor";
pub const HA_DOMAIN_SWITCH: &str = "switch";
pub const HA_DOMAIN_LIGHT: &str = "light";
pub const HA_DOMAIN_BUTTON: &str = "button";
pub const HA_DOMAIN_SELECT: &str = "select";
pub const HA_DOMAIN_NUMBER: &str = "number";
pub const HA_DOMAIN_DEVICE_TRACKER: &str = "device_tracker";

pub const HA_NUMBER_MODE_AUTO: &str = "auto";
pub const HA_NUMBER_MODE_BOX: &str = "box";
pub const HA_NUMBER_MODE_SLIDER: &str = "slider";

pub const HA_STATE_CLASS_MEASUREMENT: &str = "measurement";
pub const HA_STATE_CLASS_TOTAL: &str = "total";
pub const HA_STATE_CLASS_TOTAL_INCREASING: &str = "total_increasing";

pub const HA_DEVICE_CLASS_SENSOR_APPARENT_POWER: &str = "apparent_power";
pub const HA_DEVICE_CLASS_SENSOR_AQI: &str = "aqi";
pub const HA_DEVICE_CLASS_SENSOR_ATMOSPHERIC_PRESSURE: &str = "atmospheric_pressure";
pub const HA_DEVICE_CLASS_SENSOR_BATTERY: &str = "battery";
pub const HA_DEVICE_CLASS_SENSOR_CARBON_DIOXIDE: &str = "carbon_dioxide";
pub const HA_DEVICE_CLASS_SENSOR_CARBON_MONOXIDE: &str = "carbon_monoxide";
pub const HA_DEVICE_CLASS_SENSOR_CURRENT: &str = "current";
pub const HA_DEVICE_CLASS_SENSOR_DATA_RATE: &str = "data_rate";
pub const HA_DEVICE_CLASS_SENSOR_DATA_SIZE: &str = "data_size";
pub const HA_DEVICE_CLASS_SENSOR_DATE: &str = "date";
pub const HA_DEVICE_CLASS_SENSOR_DISTANCE: &str = "distance";
pub const HA_DEVICE_CLASS_SENSOR_DURATION: &str = "duration";
pub const HA_DEVICE_CLASS_SENSOR_ENERGY: &str = "energy";
pub const HA_DEVICE_CLASS_SENSOR_ENERGY_STORAGE: &str = "energy_storage";
pub const HA_DEVICE_CLASS_SENSOR_ENUM: &str = "enum";
pub const HA_DEVICE_CLASS_SENSOR_FREQUENCY: &str = "frequency";
pub const HA_DEVICE_CLASS_SENSOR_GAS: &str = "gas";
pub const HA_DEVICE_CLASS_SENSOR_HUMIDITY: &str = "humidity";
pub const HA_DEVICE_CLASS_SENSOR_ILLUMINANCE: &str = "illuminance";
pub const HA_DEVICE_CLASS_SENSOR_IRRADIANCE: &str = "irradiance";
pub const HA_DEVICE_CLASS_SENSOR_MOISTURE: &str = "moisture";
pub const HA_DEVICE_CLASS_SENSOR_MONETARY: &str = "monetary";
pub const HA_DEVICE_CLASS_SENSOR_NITROGEN_DIOXIDE: &str = "nitrogen_dioxide";
pub const HA_DEVICE_CLASS_SENSOR_NITROGEN_MONOXIDE: &str = "nitrogen_monoxide";
pub const HA_DEVICE_CLASS_SENSOR_NITROUS_OXIDE: &str = "nitrous_oxide";
pub const HA_DEVICE_CLASS_SENSOR_OZONE: &str = "ozone";
pub const HA_DEVICE_CLASS_SENSOR_PH: &str = "ph";
pub const HA_DEVICE_CLASS_SENSOR_PM1: &str = "pm1";
pub const HA_DEVICE_CLASS_SENSOR_PM25: &str = "pm25";
pub const HA_DEVICE_CLASS_SENSOR_PM10: &str = "pm10";
pub const HA_DEVICE_CLASS_SENSOR_POWER_FACTOR: &str = "power_factor";
pub const HA_DEVICE_CLASS_SENSOR_POWER: &str = "power";
pub const HA_DEVICE_CLASS_SENSOR_PRECIPITATION: &str = "precipitation";
pub const HA_DEVICE_CLASS_SENSOR_PRECIPITATION_INTENSITY: &str = "precipitation_intensity";
pub const HA_DEVICE_CLASS_SENSOR_PRESSURE: &str = "pressure";
pub const HA_DEVICE_CLASS_SENSOR_REACTIVE_POWER: &str = "reactive_power";
pub const HA_DEVICE_CLASS_SENSOR_SIGNAL_STRENGTH: &str = "signal_strength";
pub const HA_DEVICE_CLASS_SENSOR_SOUND_PRESSURE: &str = "sound_pressure";
pub const HA_DEVICE_CLASS_SENSOR_SPEED: &str = "speed";
pub const HA_DEVICE_CLASS_SENSOR_SULPHUR_DIOXIDE: &str = "sulphur_dioxide";
pub const HA_DEVICE_CLASS_SENSOR_TEMPERATURE: &str = "temperature";
pub const HA_DEVICE_CLASS_SENSOR_TIMESTAMP: &str = "timestamp";
pub const HA_DEVICE_CLASS_SENSOR_VOLATILE_ORGANIC_COMPOUNDS: &str = "volatile_organic_compounds";
pub const HA_DEVICE_CLASS_SENSOR_VOLATILE_ORGANIC_COMPOUNDS_PARTS: &str =
    "volatile_organic_compounds_parts";
pub const HA_DEVICE_CLASS_SENSOR_VOLTAGE: &str = "voltage";
pub const HA_DEVICE_CLASS_SENSOR_VOLUME: &str = "volume";
pub const HA_DEVICE_CLASS_SENSOR_VOLUME_FLOW_RATE: &str = "volume_flow_rate";
pub const HA_DEVICE_CLASS_SENSOR_VOLUME_STORAGE: &str = "volume_storage";
pub const HA_DEVICE_CLASS_SENSOR_WATER: &str = "water";
pub const HA_DEVICE_CLASS_SENSOR_WEIGHT: &str = "weight";
pub const HA_DEVICE_CLASS_SENSOR_WIND_SPEED: &str = "wind_speed";

pub const HA_DEVICE_CLASS_BINARY_SENSOR_BATTERY: &str = "battery";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_BATTERY_CHARGING: &str = "battery_charging";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_CARBON_MONOXIDE: &str = "carbon_monoxide";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_COLD: &str = "cold";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_CONNECTIVITY: &str = "connectivity";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_DOOR: &str = "door";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_GARAGE_DOOR: &str = "garage_door";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_GAS: &str = "gas";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_HEAT: &str = "heat";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_LIGHT: &str = "light";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_LOCK: &str = "lock";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_MOISTURE: &str = "moisture";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_MOTION: &str = "motion";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_MOVING: &str = "moving";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_OCCUPANCY: &str = "occupancy";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_OPENING: &str = "opening";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_PLUG: &str = "plug";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_POWER: &str = "power";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_PRESENCE: &str = "presence";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_PROBLEM: &str = "problem";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_RUNNING: &str = "running";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_SAFETY: &str = "safety";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_SMOKE: &str = "smoke";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_SOUND: &str = "sound";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_TAMPER: &str = "tamper";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_UPDATE: &str = "update";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_VIBRATION: &str = "vibration";
pub const HA_DEVICE_CLASS_BINARY_SENSOR_WINDOW: &str = "window";

pub const HA_DEVICE_CLASS_BUTTON_IDENTIFY: &str = "identify";
pub const HA_DEVICE_CLASS_BUTTON_RESTART: &str = "restart";
pub const HA_DEVICE_CLASS_BUTTON_UPDATE: &str = "update";

pub const HA_DEVICE_CLASS_SWITCH_OUTLET: &str = "outlet";
pub const HA_DEVICE_CLASS_SWITCH_SWITCH: &str = "switch";

pub const HA_DEVICE_CLASS_NUMBER_APPARENT_POWER: &str = "apparent_power";
pub const HA_DEVICE_CLASS_NUMBER_AQI: &str = "aqi";
pub const HA_DEVICE_CLASS_NUMBER_ATMOSPHERIC_PRESSURE: &str = "atmospheric_pressure";
pub const HA_DEVICE_CLASS_NUMBER_BATTERY: &str = "battery";
pub const HA_DEVICE_CLASS_NUMBER_CARBON_DIOXIDE: &str = "carbon_dioxide";
pub const HA_DEVICE_CLASS_NUMBER_CARBON_MONOXIDE: &str = "carbon_monoxide";
pub const HA_DEVICE_CLASS_NUMBER_CURRENT: &str = "current";
pub const HA_DEVICE_CLASS_NUMBER_DATA_RATE: &str = "data_rate";
pub const HA_DEVICE_CLASS_NUMBER_DATA_SIZE: &str = "data_size";
pub const HA_DEVICE_CLASS_NUMBER_DISTANCE: &str = "distance";
pub const HA_DEVICE_CLASS_NUMBER_DURATION: &str = "duration";
pub const HA_DEVICE_CLASS_NUMBER_ENERGY: &str = "energy";
pub const HA_DEVICE_CLASS_NUMBER_FREQUENCY: &str = "frequency";
pub const HA_DEVICE_CLASS_NUMBER_GAS: &str = "gas";
pub const HA_DEVICE_CLASS_NUMBER_HUMIDITY: &str = "humidity";
pub const HA_DEVICE_CLASS_NUMBER_ILLUMINANCE: &str = "illuminance";
pub const HA_DEVICE_CLASS_NUMBER_IRRADIANCE: &str = "irradiance";
pub const HA_DEVICE_CLASS_NUMBER_MOISTURE: &str = "moisture";
pub const HA_DEVICE_CLASS_NUMBER_MONETARY: &str = "monetary";
pub const HA_DEVICE_CLASS_NUMBER_NITROGEN_DIOXIDE: &str = "nitrogen_dioxide";
pub const HA_DEVICE_CLASS_NUMBER_NITROGEN_MONOXIDE: &str = "nitrogen_monoxide";
pub const HA_DEVICE_CLASS_NUMBER_NITROUS_OXIDE: &str = "nitrous_oxide";
pub const HA_DEVICE_CLASS_NUMBER_OZONE: &str = "ozone";
pub const HA_DEVICE_CLASS_NUMBER_PH: &str = "ph";
pub const HA_DEVICE_CLASS_NUMBER_PM1: &str = "pm1";
pub const HA_DEVICE_CLASS_NUMBER_PM25: &str = "pm25";
pub const HA_DEVICE_CLASS_NUMBER_PM10: &str = "pm10";
pub const HA_DEVICE_CLASS_NUMBER_POWER_FACTOR: &str = "power_factor";
pub const HA_DEVICE_CLASS_NUMBER_POWER: &str = "power";
pub const HA_DEVICE_CLASS_NUMBER_PRECIPITATION: &str = "precipitation";
pub const HA_DEVICE_CLASS_NUMBER_PRECIPITATION_INTENSITY: &str = "precipitation_intensity";
pub const HA_DEVICE_CLASS_NUMBER_PRESSURE: &str = "pressure";
pub const HA_DEVICE_CLASS_NUMBER_REACTIVE_POWER: &str = "reactive_power";
pub const HA_DEVICE_CLASS_NUMBER_SIGNAL_STRENGTH: &str = "signal_strength";
pub const HA_DEVICE_CLASS_NUMBER_SOUND_PRESSURE: &str = "sound_pressure";
pub const HA_DEVICE_CLASS_NUMBER_SPEED: &str = "speed";
pub const HA_DEVICE_CLASS_NUMBER_SULPHUR_DIOXIDE: &str = "sulphur_dioxide";
pub const HA_DEVICE_CLASS_NUMBER_TEMPERATURE: &str = "temperature";
pub const HA_DEVICE_CLASS_NUMBER_VOLATILE_ORGANIC_COMPOUNDS: &str = "volatile_organic_compounds";
pub const HA_DEVICE_CLASS_NUMBER_VOLATILE_ORGANIC_COMPOUNDS_PARTS: &str =
    "volatile_organic_compounds_parts";
pub const HA_DEVICE_CLASS_NUMBER_VOLTAGE: &str = "voltage";
pub const HA_DEVICE_CLASS_NUMBER_VOLUME: &str = "volume";
pub const HA_DEVICE_CLASS_NUMBER_WATER: &str = "water";
pub const HA_DEVICE_CLASS_NUMBER_WEIGHT: &str = "weight";
pub const HA_DEVICE_CLASS_NUMBER_WIND_SPEED: &str = "wind_speed";

pub const HA_UNIT_TEMPERATURE_CELSIUS: &str = "°C";
pub const HA_UNIT_TEMPERATURE_KELVIN: &str = "K";
pub const HA_UNIT_TEMPERATURE_FAHRENHEIT: &str = "°F";

pub const HA_UNIT_TIME_MILLISECONDS: &str = "ms";
pub const HA_UNIT_TIME_SECONDS: &str = "s";
pub const HA_UNIT_TIME_MINUTES: &str = "min";
pub const HA_UNIT_TIME_HOURS: &str = "h";
pub const HA_UNIT_TIME_DAYS: &str = "d";

pub const HA_UNIT_PERCENTAGE: &str = "%";

pub const HA_UNIT_POWER_WATT: &str = "W";
pub const HA_UNIT_POWER_KILOWATT: &str = "kW";

pub const HA_UNIT_VOLTAGE_VOLT: &str = "V";
pub const HA_UNIT_CURRENT_AMPERE: &str = "A";

pub const HA_UNIT_DISTANCE_MILLIMETER: &str = "mm";
pub const HA_UNIT_DISTANCE_CENTIMETER: &str = "cm";
pub const HA_UNIT_DISTANCE_METER: &str = "m";
pub const HA_UNIT_DISTANCE_KILOMETER: &str = "km";

pub const HA_UNIT_CURRENCY_USD: &str = "USD";
pub const HA_UNIT_CURRENCY_EUR: &str = "EUR";
pub const HA_UNIT_CURRENCY_GBP: &str = "GBP";
pub const HA_UNIT_CURRENCY_JPY: &str = "JPY";
pub const HA_UNIT_CURRENCY_CNY: &str = "CNY";
pub const HA_UNIT_CURRENCY_CAD: &str = "CAD";
pub const HA_UNIT_CURRENCY_AUD: &str = "AUD";
pub const HA_UNIT_CURRENCY_CHF: &str = "CHF";
pub const HA_UNIT_CURRENCY_INR: &str = "INR";
pub const HA_UNIT_CURRENCY_BRL: &str = "BRL";

pub const HA_UNIT_CURRENCY_DOLLAR: &str = "$";
pub const HA_UNIT_CURRENCY_EURO: &str = "€";
pub const HA_UNIT_CURRENCY_POUND: &str = "£";
pub const HA_UNIT_CURRENCY_YEN: &str = "¥";
pub const HA_UNIT_CURRENCY_CENT: &str = "¢";

pub const HA_ENTITY_CATEGORY_CONFIG: &str = "config";
pub const HA_ENTITY_CATEGORY_DIAGNOSTIC: &str = "diagnostic";

pub const HA_BINARY_STATE_ON: &str = "ON";
pub const HA_BINARY_STATE_OFF: &str = "OFF";

pub const HA_SWITCH_STATE_ON: &str = "ON";
pub const HA_SWITCH_STATE_OFF: &str = "OFF";

pub const HA_BINARY_SENSOR_STATE_ON: &str = "ON";
pub const HA_BINARY_SENSOR_STATE_OFF: &str = "OFF";

pub const HA_BUTTON_PAYLOAD_PRESS: &str = "PRESS";

// Number units - Energy
pub const HA_UNIT_ENERGY_JOULE: &str = "J";
pub const HA_UNIT_ENERGY_KILOJOULE: &str = "kJ";
pub const HA_UNIT_ENERGY_MEGAJOULE: &str = "MJ";
pub const HA_UNIT_ENERGY_GIGAJOULE: &str = "GJ";
pub const HA_UNIT_ENERGY_MILLIWATTHOUR: &str = "mWh";
pub const HA_UNIT_ENERGY_WATTHOUR: &str = "Wh";
pub const HA_UNIT_ENERGY_KWH: &str = "kWh";
pub const HA_UNIT_ENERGY_MEGAWATTHOUR: &str = "MWh";
pub const HA_UNIT_ENERGY_GIGAWATTHOUR: &str = "GWh";
pub const HA_UNIT_ENERGY_TERAWATTHOUR: &str = "TWh";
pub const HA_UNIT_ENERGY_CALORIE: &str = "cal";
pub const HA_UNIT_ENERGY_KILOCALORIE: &str = "kcal";
pub const HA_UNIT_ENERGY_MEGACALORIE: &str = "Mcal";
pub const HA_UNIT_ENERGY_GIGACALORIE: &str = "Gcal";

// Number units - Pressure
pub const HA_UNIT_PRESSURE_MILLIPASCAL: &str = "mPa";
pub const HA_UNIT_PRESSURE_PASCAL: &str = "Pa";
pub const HA_UNIT_PRESSURE_HPA: &str = "hPa";
pub const HA_UNIT_PRESSURE_KILOPASCAL: &str = "kPa";
pub const HA_UNIT_PRESSURE_BAR: &str = "bar";
pub const HA_UNIT_PRESSURE_CENTIBAR: &str = "cbar";
pub const HA_UNIT_PRESSURE_MILLIBAR: &str = "mbar";
pub const HA_UNIT_PRESSURE_MILLIMETER_MERCURY: &str = "mmHg";
pub const HA_UNIT_PRESSURE_INCH_MERCURY: &str = "inHg";
pub const HA_UNIT_PRESSURE_INCH_WATER: &str = "inH₂O";
pub const HA_UNIT_PRESSURE_PSI: &str = "psi";

// Number units - Volume
pub const HA_UNIT_VOLUME_LITER: &str = "L";
pub const HA_UNIT_VOLUME_MILLILITER: &str = "mL";
pub const HA_UNIT_VOLUME_GALLON: &str = "gal";
pub const HA_UNIT_VOLUME_FLUID_OUNCE: &str = "fl. oz.";
pub const HA_UNIT_VOLUME_CUBIC_METER: &str = "m³";
pub const HA_UNIT_VOLUME_CUBIC_FOOT: &str = "ft³";
pub const HA_UNIT_VOLUME_CCF: &str = "CCF";
pub const HA_UNIT_VOLUME_MCF: &str = "MCF";

// Number units - Speed
pub const HA_UNIT_SPEED_FEET_PER_SECOND: &str = "ft/s";
pub const HA_UNIT_SPEED_INCH_PER_DAY: &str = "in/d";
pub const HA_UNIT_SPEED_INCH_PER_HOUR: &str = "in/h";
pub const HA_UNIT_SPEED_INCH_PER_SECOND: &str = "in/s";
pub const HA_UNIT_SPEED_KILOMETER_PER_HOUR: &str = "km/h";
pub const HA_UNIT_SPEED_KNOT: &str = "kn";
pub const HA_UNIT_SPEED_METER_PER_SECOND: &str = "m/s";
pub const HA_UNIT_SPEED_MILE_PER_HOUR: &str = "mph";
pub const HA_UNIT_SPEED_MILLIMETER_PER_DAY: &str = "mm/d";
pub const HA_UNIT_SPEED_MILLIMETER_PER_SECOND: &str = "mm/s";

// Number units - Distance (additional to existing)
pub const HA_UNIT_DISTANCE_MILE: &str = "mi";
pub const HA_UNIT_DISTANCE_NAUTICAL_MILE: &str = "nmi";
pub const HA_UNIT_DISTANCE_YARD: &str = "yd";
pub const HA_UNIT_DISTANCE_INCH: &str = "in";

// Number units - Power (additional)
pub const HA_UNIT_POWER_MILLIWATT: &str = "mW";
pub const HA_UNIT_POWER_MEGAWATT: &str = "MW";
pub const HA_UNIT_POWER_GIGAWATT: &str = "GW";
pub const HA_UNIT_POWER_TERAWATT: &str = "TW";

// Number units - Current (additional)
pub const HA_UNIT_CURRENT_MILLIAMPERE: &str = "mA";

// Number units - Voltage (additional)
pub const HA_UNIT_VOLTAGE_MILLIVOLT: &str = "mV";
pub const HA_UNIT_VOLTAGE_MICROVOLT: &str = "µV";
pub const HA_UNIT_VOLTAGE_KILOVOLT: &str = "kV";
pub const HA_UNIT_VOLTAGE_MEGAVOLT: &str = "MV";

// Number units - Data Rate
pub const HA_UNIT_DATA_RATE_BIT_PER_SECOND: &str = "bit/s";
pub const HA_UNIT_DATA_RATE_KILOBIT_PER_SECOND: &str = "kbit/s";
pub const HA_UNIT_DATA_RATE_MEGABIT_PER_SECOND: &str = "Mbit/s";
pub const HA_UNIT_DATA_RATE_GIGABIT_PER_SECOND: &str = "Gbit/s";
pub const HA_UNIT_DATA_RATE_BYTE_PER_SECOND: &str = "B/s";
pub const HA_UNIT_DATA_RATE_KILOBYTE_PER_SECOND: &str = "kB/s";
pub const HA_UNIT_DATA_RATE_MEGABYTE_PER_SECOND: &str = "MB/s";
pub const HA_UNIT_DATA_RATE_GIGABYTE_PER_SECOND: &str = "GB/s";
pub const HA_UNIT_DATA_RATE_KIBIBYTE_PER_SECOND: &str = "KiB/s";
pub const HA_UNIT_DATA_RATE_MEBIBYTE_PER_SECOND: &str = "MiB/s";
pub const HA_UNIT_DATA_RATE_GIBIBYTE_PER_SECOND: &str = "GiB/s";

// Number units - Weight
pub const HA_UNIT_WEIGHT_KILOGRAM: &str = "kg";
pub const HA_UNIT_WEIGHT_GRAM: &str = "g";
pub const HA_UNIT_WEIGHT_MILLIGRAM: &str = "mg";
pub const HA_UNIT_WEIGHT_MICROGRAM: &str = "µg";
pub const HA_UNIT_WEIGHT_OUNCE: &str = "oz";
pub const HA_UNIT_WEIGHT_POUND: &str = "lb";
pub const HA_UNIT_WEIGHT_STONE: &str = "st";

// Light
pub const HA_UNIT_LIGHT_LUX: &str = "lx";

// Signal Strength
pub const HA_UNIT_SIGNAL_STRENGTH_DBM: &str = "dBm";
