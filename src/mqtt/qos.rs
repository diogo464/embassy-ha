#[derive(Debug)]
pub struct InvalidQos(u8);

impl core::fmt::Display for InvalidQos {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid QoS value: '{}'", self.0)
    }
}

impl core::error::Error for InvalidQos {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Qos {
    #[default]
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}

impl core::fmt::Display for Qos {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Qos::AtMostOnce => "QoS::AtMostOnce",
            Qos::AtLeastOnce => "Qos::AtLeastOnce",
            Qos::ExactlyOnce => "Qos::ExactlyOnce",
        })
    }
}

impl Qos {
    pub fn to_u8(&self) -> u8 {
        match self {
            Qos::AtMostOnce => 0,
            Qos::AtLeastOnce => 1,
            Qos::ExactlyOnce => 2,
        }
    }

    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::AtMostOnce),
            1 => Some(Self::AtLeastOnce),
            2 => Some(Self::ExactlyOnce),
            _ => None,
        }
    }
}

impl TryFrom<u8> for Qos {
    type Error = InvalidQos;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match Self::from_u8(value) {
            Some(v) => Ok(v),
            None => Err(InvalidQos(value)),
        }
    }
}

impl From<Qos> for u8 {
    fn from(value: Qos) -> Self {
        value.to_u8()
    }
}
