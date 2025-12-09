#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PacketId(u16);

impl From<u16> for PacketId {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<PacketId> for u16 {
    fn from(value: PacketId) -> Self {
        value.0
    }
}
