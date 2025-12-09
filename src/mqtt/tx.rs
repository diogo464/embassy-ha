use super::{
    PacketId,
    field::{self, Field, FieldBuffer},
    protocol,
    qos::Qos,
};

pub struct Connect<'a> {
    pub client_id: &'a str,
    pub clean_session: bool,
    pub username: Option<&'a str>,
    pub password: Option<&'a [u8]>,
    pub will_topic: Option<&'a str>,
    pub will_payload: Option<&'a [u8]>,
    pub will_retain: bool,
    pub keepalive: Option<u16>,
}

pub fn connect<'a>(buffer: &mut FieldBuffer<'a>, connect: Connect<'a>) {
    let mut flags = 0;
    if connect.clean_session {
        flags |= protocol::CONNECT_FLAG_CLEAN_SESSION;
    }
    if connect.username.is_some() {
        flags |= protocol::CONNECT_FLAG_USERNAME;
    }
    if connect.password.is_some() {
        flags |= protocol::CONNECT_FLAG_PASSWORD;
    }
    if connect.will_topic.is_some() {
        flags |= protocol::CONNECT_FLAG_WILL_FLAG;
    }
    if connect.will_retain {
        flags |= protocol::CONNECT_FLAG_WILL_RETAIN;
    }

    buffer.push(Field::U8(protocol::create_header_control(
        protocol::PACKET_TYPE_CONNECT,
        0,
    )));
    buffer.push(Field::VarInt(0));

    buffer.push(Field::LenPrefixedString(protocol::PROTOCOL_NAME));
    buffer.push(Field::U8(protocol::PROTOCOL_LEVEL_3_1_1));
    buffer.push(Field::U8(flags));
    buffer.push(Field::U16(connect.keepalive.unwrap_or(0)));
    buffer.push(Field::LenPrefixedString(connect.client_id));
    if let Some(will_topic) = connect.will_topic {
        buffer.push(Field::LenPrefixedString(will_topic));
        buffer.push(Field::LenPrefixedBuffer(
            connect.will_payload.unwrap_or(&[]),
        ));
    }
    if let Some(username) = connect.username {
        buffer.push(Field::LenPrefixedString(username));
    }
    if let Some(password) = connect.password {
        buffer.push(Field::LenPrefixedBuffer(password));
    }

    let message_size = field::fields_size(&buffer.as_slice()[2..]);
    buffer.set(1, Field::VarInt(u32::try_from(message_size).unwrap()));
}

pub struct Publish<'a> {
    pub topic: &'a str,
    pub payload: &'a [u8],
    pub qos: Qos,
    pub retain: bool,
    pub dup: bool,
    pub packet_id: Option<PacketId>,
}

pub fn publish<'a>(buffer: &mut FieldBuffer<'a>, publish: Publish<'a>) {
    let mut flags = 0u8;

    // Set QoS bits (bits 1-2)
    flags |= (publish.qos.to_u8() & 0x03) << 1;

    // Set RETAIN flag (bit 0)
    if publish.retain {
        flags |= 0x01;
    }

    // Set DUP flag (bit 3)
    if publish.dup {
        flags |= 0x08;
    }

    buffer.push(Field::U8(protocol::create_header_control(
        protocol::PACKET_TYPE_PUBLISH,
        flags,
    )));
    buffer.push(Field::VarInt(0));

    buffer.push(Field::LenPrefixedString(publish.topic));

    // Packet ID is only present for QoS 1 and 2
    if publish.qos.to_u8() > 0 {
        // TODO: turn this into a warning
        let packet_id = publish.packet_id.expect("packet_id required for QoS > 0");
        buffer.push(Field::U16(packet_id.into()));
    }

    buffer.push(Field::Buffer(publish.payload));

    let message_size = field::fields_size(&buffer.as_slice()[2..]);
    buffer.set(1, Field::VarInt(u32::try_from(message_size).unwrap()));
}

pub struct Subscribe<'a> {
    pub topic: &'a str,
    pub qos: Qos,
    pub packet_id: PacketId,
}

pub fn subscribe<'a>(buffer: &mut FieldBuffer<'a>, subscribe: Subscribe<'a>) {
    // SUBSCRIBE packets have fixed header flags (reserved bits)
    buffer.push(Field::U8(protocol::create_header_control(
        protocol::PACKET_TYPE_SUBSCRIBE,
        protocol::SUBSCRIBE_HEADER_FLAGS,
    )));
    buffer.push(Field::VarInt(0));

    // Variable header: packet identifier
    buffer.push(Field::U16(subscribe.packet_id.into()));

    // Payload: topic filter + QoS
    buffer.push(Field::LenPrefixedString(subscribe.topic));
    buffer.push(Field::U8(subscribe.qos.to_u8()));

    let message_size = field::fields_size(&buffer.as_slice()[2..]);
    buffer.set(1, Field::VarInt(u32::try_from(message_size).unwrap()));
}

pub struct Unsubscribe<'a> {
    pub topic: &'a str,
    pub packet_id: PacketId,
}

pub fn unsubscribe<'a>(buffer: &mut FieldBuffer<'a>, unsubscribe: Unsubscribe<'a>) {
    // UNSUBSCRIBE packets have fixed header flags (reserved bits)
    buffer.push(Field::U8(protocol::create_header_control(
        protocol::PACKET_TYPE_UNSUBSCRIBE,
        protocol::UNSUBSCRIBE_HEADER_FLAGS,
    )));
    buffer.push(Field::VarInt(0));

    // Variable header: packet identifier
    buffer.push(Field::U16(unsubscribe.packet_id.into()));

    // Payload: topic filter (no QoS)
    buffer.push(Field::LenPrefixedString(unsubscribe.topic));

    let message_size = field::fields_size(&buffer.as_slice()[2..]);
    buffer.set(1, Field::VarInt(u32::try_from(message_size).unwrap()));
}

pub fn disconnect(buffer: &mut FieldBuffer) {
    // DISCONNECT has no variable header or payload
    buffer.push(Field::U8(protocol::create_header_control(
        protocol::PACKET_TYPE_DISCONNECT,
        0,
    )));
    buffer.push(Field::VarInt(0));
}

pub fn puback(buffer: &mut FieldBuffer, packet_id: PacketId) {
    buffer.push(Field::U8(protocol::create_header_control(
        protocol::PACKET_TYPE_PUBACK,
        0,
    )));
    buffer.push(Field::VarInt(2)); // Remaining length is always 2 (packet ID)
    buffer.push(Field::U16(packet_id.into()));
}

pub fn pubrec(buffer: &mut FieldBuffer, packet_id: PacketId) {
    buffer.push(Field::U8(protocol::create_header_control(
        protocol::PACKET_TYPE_PUBREC,
        0,
    )));
    buffer.push(Field::VarInt(2)); // Remaining length is always 2 (packet ID)
    buffer.push(Field::U16(packet_id.into()));
}

pub fn pubrel(buffer: &mut FieldBuffer, packet_id: PacketId) {
    buffer.push(Field::U8(protocol::create_header_control(
        protocol::PACKET_TYPE_PUBREL,
        protocol::PUBREL_HEADER_FLAGS,
    )));
    buffer.push(Field::VarInt(2)); // Remaining length is always 2 (packet ID)
    buffer.push(Field::U16(packet_id.into()));
}

pub fn pubcomp(buffer: &mut FieldBuffer, packet_id: PacketId) {
    buffer.push(Field::U8(protocol::create_header_control(
        protocol::PACKET_TYPE_PUBCOMP,
        0,
    )));
    buffer.push(Field::VarInt(2)); // Remaining length is always 2 (packet ID)
    buffer.push(Field::U16(packet_id.into()));
}

