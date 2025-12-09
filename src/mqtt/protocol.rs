pub const PACKET_TYPE_CONNECT: u8 = 1;
pub const PACKET_TYPE_CONNACK: u8 = 2;
pub const PACKET_TYPE_PUBLISH: u8 = 3;
pub const PACKET_TYPE_PUBACK: u8 = 4;
pub const PACKET_TYPE_PUBREC: u8 = 5;
pub const PACKET_TYPE_PUBREL: u8 = 6;
pub const PACKET_TYPE_PUBCOMP: u8 = 7;
pub const PACKET_TYPE_SUBSCRIBE: u8 = 8;
pub const PACKET_TYPE_SUBACK: u8 = 9;
pub const PACKET_TYPE_UNSUBSCRIBE: u8 = 10;
pub const PACKET_TYPE_UNSUBACK: u8 = 11;
pub const PACKET_TYPE_PINGREQ: u8 = 12;
pub const PACKET_TYPE_PINGRESP: u8 = 13;
pub const PACKET_TYPE_DISCONNECT: u8 = 14;

pub const PROTOCOL_NAME: &str = "MQTT";

pub const PROTOCOL_LEVEL_3_1_1: u8 = 0x04;
pub const PROTOCOL_LEVEL_5_0_0: u8 = 0x05;

pub const CONNECT_FLAG_USERNAME: u8 = 1 << 7;
pub const CONNECT_FLAG_PASSWORD: u8 = 1 << 6;
pub const CONNECT_FLAG_WILL_RETAIN: u8 = 1 << 5;
pub const CONNECT_FLAG_WILL_FLAG: u8 = 1 << 2;
pub const CONNECT_FLAG_CLEAN_SESSION: u8 = 1 << 1;

pub const SUBSCRIBE_HEADER_FLAGS: u8 = 0x02;
pub const UNSUBSCRIBE_HEADER_FLAGS: u8 = 0x02;
pub const PUBREL_HEADER_FLAGS: u8 = 0x02;

pub const CONNACK_CODE_ACCEPTED: u8 = 0;
pub const CONNACK_CODE_UNACCEPTABLE_PROTOCOL_VERSION: u8 = 1;
pub const CONNACK_CODE_IDENTIFIER_REJECTED: u8 = 2;
pub const CONNACK_CODE_SERVER_UNAVAILABLE: u8 = 3;
pub const CONNACK_CODE_BAD_USERNAME_PASSWORD: u8 = 4;
pub const CONNACK_CODE_NOT_AUTHORIZED: u8 = 5;

pub const CONNACK_FLAG_SESSION_PRESENT: u8 = 0x01;
pub const CONNACK_FLAG_RESERVED: u8 = 0xFE;

pub const SUBACK_FAILURE: u8 = 0x80;

pub const PUBLISH_FLAG_RETAIN: u8 = 0x01;
pub const PUBLISH_FLAG_QOS_MASK: u8 = 0x06;
pub const PUBLISH_FLAG_QOS_SHIFT: u8 = 1;
pub const PUBLISH_FLAG_DUP: u8 = 0x08;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeaderControl {
    pub packet_type: u8,
    pub packet_flags: u8,
}

pub fn create_header_control(packet_type: u8, flags: u8) -> u8 {
    assert!(packet_type & 0xF0 == 0);
    assert!(flags & 0xF0 == 0);
    packet_type << 4 | flags
}

pub fn split_header_control(control: u8) -> HeaderControl {
    HeaderControl {
        packet_type: control >> 4,
        packet_flags: control & 0x0F,
    }
}
