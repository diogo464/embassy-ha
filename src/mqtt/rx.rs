use super::{ConnectCode, PacketId, Qos, protocol, varint};

#[derive(Debug)]
pub enum Error {
    NeedMoreData,
    InvalidPacket(&'static str),
    UnsupportedPacket { packet_type: u8, packet_len: u32 },
    UnknownPacket { packet_type: u8, packet_len: u32 },
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::NeedMoreData => f.write_str("need more data"),
            Error::InvalidPacket(msg) => write!(f, "invalid packet: {}", msg),
            Error::UnsupportedPacket {
                packet_type,
                packet_len,
            } => write!(
                f,
                "unsupported packet type {} with length {}",
                packet_type, packet_len
            ),
            Error::UnknownPacket {
                packet_type,
                packet_len,
            } => write!(
                f,
                "unknown packet type {} with length {}",
                packet_type, packet_len
            ),
        }
    }
}

impl From<varint::Error> for Error {
    fn from(value: varint::Error) -> Self {
        match value {
            varint::Error::NeedMoreData => Self::NeedMoreData,
            varint::Error::InvalidVarInt => {
                Self::InvalidPacket("invalid variable integer encoding")
            }
        }
    }
}

pub enum Packet<'a> {
    ConnAck {
        session_present: bool,
        code: ConnectCode,
    },
    Publish {
        topic: &'a str,
        packet_id: Option<PacketId>,
        qos: Qos,
        retain: bool,
        dup: bool,
        data_len: usize,
    },
    PubAck {
        packet_id: PacketId,
    },
    SubscribeAck {
        packet_id: PacketId,
        success: bool,
    },
    UnsubscribeAck {
        packet_id: PacketId,
    },
    PingResp,
}

pub fn decode<'a>(buf: &'a [u8]) -> Result<(Packet<'a>, usize), Error> {
    let mut reader = Reader::new(buf);
    let protocol::HeaderControl {
        packet_type,
        packet_flags,
    } = protocol::split_header_control(reader.read_u8()?);
    let packet_len = reader.read_varint()?;

    let packet = match packet_type {
        protocol::PACKET_TYPE_CONNACK => {
            let flags = reader.read_u8()?;
            let code = ConnectCode::from(reader.read_u8()?);
            let session_present = flags & protocol::CONNACK_FLAG_SESSION_PRESENT != 0;
            if flags & protocol::CONNACK_FLAG_RESERVED != 0 {
                return Err(Error::InvalidPacket("CONNACK reserved flags must be zero"));
            }
            Packet::ConnAck {
                session_present,
                code,
            }
        }
        protocol::PACKET_TYPE_PUBLISH => {
            // Extract flags from the fixed header
            let retain = (packet_flags & protocol::PUBLISH_FLAG_RETAIN) != 0;
            let qos_value = (packet_flags & protocol::PUBLISH_FLAG_QOS_MASK)
                >> protocol::PUBLISH_FLAG_QOS_SHIFT;
            let qos = Qos::from_u8(qos_value)
                .ok_or(Error::InvalidPacket("PUBLISH has invalid QoS value"))?;
            let dup = (packet_flags & protocol::PUBLISH_FLAG_DUP) != 0;

            // Track position after fixed header to calculate data length
            let variable_header_start = reader.num_read();

            // Read topic name
            let topic = reader.read_len_prefix_str()?;

            // Read packet ID if QoS > 0
            let packet_id = if qos.to_u8() > 0 {
                Some(PacketId::from(reader.read_u16()?))
            } else {
                None
            };

            // Calculate payload length without reading it
            let variable_header_len = reader.num_read() - variable_header_start;
            let data_len = (packet_len as usize)
                .checked_sub(variable_header_len)
                .ok_or(Error::InvalidPacket(
                    "PUBLISH remaining length is too short for headers",
                ))?;

            Packet::Publish {
                topic,
                packet_id,
                qos,
                retain,
                dup,
                data_len,
            }
        }
        protocol::PACKET_TYPE_PUBACK => {
            if packet_flags != 0 {
                return Err(Error::InvalidPacket("PUBACK flags must be zero"));
            }
            if packet_len != 2 {
                return Err(Error::InvalidPacket("PUBACK remaining length must be 2"));
            }
            let packet_id = PacketId::from(reader.read_u16()?);
            Packet::PubAck { packet_id }
        }
        protocol::PACKET_TYPE_SUBACK => {
            if packet_flags != 0 {
                return Err(Error::InvalidPacket("SUBACK flags must be zero"));
            }
            if packet_len < 3 {
                // Minimum: 2 bytes packet ID + 1 byte return code
                return Err(Error::InvalidPacket(
                    "SUBACK remaining length must be at least 3",
                ));
            }
            let packet_id = PacketId::from(reader.read_u16()?);
            let return_code = reader.read_u8()?;
            let success = return_code != protocol::SUBACK_FAILURE;
            Packet::SubscribeAck { packet_id, success }
        }
        protocol::PACKET_TYPE_UNSUBACK => {
            if packet_flags != 0 {
                return Err(Error::InvalidPacket("UNSUBACK flags must be zero"));
            }
            if packet_len != 2 {
                return Err(Error::InvalidPacket("UNSUBACK remaining length must be 2"));
            }
            let packet_id = PacketId::from(reader.read_u16()?);
            Packet::UnsubscribeAck { packet_id }
        }
        protocol::PACKET_TYPE_PINGRESP => {
            if packet_flags != 0 {
                return Err(Error::InvalidPacket("PINGRESP flags must be zero"));
            }
            if packet_len != 0 {
                return Err(Error::InvalidPacket("PINGRESP remaining length must be 0"));
            }
            Packet::PingResp
        }
        protocol::PACKET_TYPE_CONNECT
        | protocol::PACKET_TYPE_PUBREC
        | protocol::PACKET_TYPE_PUBREL
        | protocol::PACKET_TYPE_PUBCOMP
        | protocol::PACKET_TYPE_DISCONNECT
        | protocol::PACKET_TYPE_SUBSCRIBE
        | protocol::PACKET_TYPE_UNSUBSCRIBE
        | protocol::PACKET_TYPE_PINGREQ => {
            return Err(Error::UnsupportedPacket {
                packet_type,
                packet_len,
            });
        }
        _ => {
            return Err(Error::UnknownPacket {
                packet_type,
                packet_len,
            });
        }
    };

    Ok((packet, reader.num_read()))
}

struct Reader<'a> {
    buf: &'a [u8],
    off: usize,
}

impl<'a> Reader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, off: 0 }
    }

    fn remain(&self) -> usize {
        self.buf.len() - self.off
    }

    fn remain_slice(&self) -> &'a [u8] {
        &self.buf[self.off..]
    }

    fn num_read(&self) -> usize {
        self.off
    }

    fn read_buf(&mut self, n: usize) -> Result<&'a [u8], Error> {
        if self.remain() < n {
            return Err(Error::NeedMoreData);
        }
        let v = &self.buf[self.off..self.off + n];
        self.off += n;
        Ok(v)
    }

    fn read_u8(&mut self) -> Result<u8, Error> {
        let v = self.read_buf(1)?;
        Ok(v[0])
    }

    fn read_u16(&mut self) -> Result<u16, Error> {
        let v = self.read_buf(2)?;
        Ok(u16::from_be_bytes([v[0], v[1]]))
    }

    fn read_len_prefix_buf(&mut self) -> Result<&'a [u8], Error> {
        let l = self.read_u16()?;
        let v = self.read_buf(usize::from(l))?;
        Ok(v)
    }

    fn read_len_prefix_str(&mut self) -> Result<&'a str, Error> {
        let v = self.read_len_prefix_buf()?;
        Ok(str::from_utf8(v).unwrap())
    }

    fn read_varint(&mut self) -> Result<u32, Error> {
        let (value, consumed) = varint::decode(self.remain_slice())?;
        self.off += consumed;
        Ok(value)
    }
}
