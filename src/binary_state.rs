use core::str::FromStr;

use crate::constants;

pub struct InvalidBinaryState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryState {
    On,
    Off,
}

impl BinaryState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::On => constants::HA_SWITCH_STATE_ON,
            Self::Off => constants::HA_SWITCH_STATE_OFF,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

impl core::fmt::Display for BinaryState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for BinaryState {
    type Err = InvalidBinaryState;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case(constants::HA_SWITCH_STATE_ON) {
            return Ok(Self::On);
        }
        if s.eq_ignore_ascii_case(constants::HA_SWITCH_STATE_OFF) {
            return Ok(Self::Off);
        }
        Err(InvalidBinaryState)
    }
}

impl TryFrom<&[u8]> for BinaryState {
    type Error = InvalidBinaryState;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let string = str::from_utf8(value).map_err(|_| InvalidBinaryState)?;
        string.parse()
    }
}
