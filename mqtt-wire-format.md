# MQTT Wire Format for Home Assistant

This document covers the essential MQTT wire format details needed for implementing Home Assistant's MQTT integration.

## Overview

MQTT uses a binary protocol over TCP. Each message consists of:
1. **Fixed Header** (always present)
2. **Variable Header** (present in most message types)
3. **Payload** (optional, depends on message type)

## Message Flow: When to Wait for Responses

Understanding which messages require broker responses is critical for implementing the protocol correctly.

### Client-Initiated Messages

| Client Sends                            | Broker Responds                         | Must Wait?                              | Notes                                   |
| --------------------------------------- | --------------------------------------- | --------------------------------------- | --------------------------------------- |
| CONNECT                                 | CONNACK                                 | ✅ Yes                                 | Must wait before sending other messages |
| PUBLISH QoS 0                           | *(none)*                                | ❌ No                                  | Fire and forget                         |
| PUBLISH QoS 1                           | PUBACK                                  | ✅ Yes                                 | Wait for acknowledgment                 |
| PUBLISH QoS 2                           | PUBREC → PUBREL → PUBCOMP           | ✅ Yes                                 | Four-way handshake                      |
| SUBSCRIBE                               | SUBACK                                  | ✅ Yes                                 | Contains subscription result codes      |
| UNSUBSCRIBE                             | UNSUBACK                                | ✅ Yes                                 | Confirms unsubscription                 |
| PINGREQ                                 | PINGRESP                                | ✅ Yes                                 | Keep-alive mechanism                    |
| DISCONNECT                              | *(none)*                                | ❌ No                                  | Graceful shutdown                       |


### Broker-Initiated Messages

The broker can send these messages to the client at any time:

| Broker Sends | When | Client Action |
|--------------|------|---------------|
| PUBLISH | When a message arrives on a subscribed topic | Send PUBACK if QoS 1, or PUBREC if QoS 2 |
| PINGRESP | In response to PINGREQ | No further action needed |

### Implementation Notes

1. **CONNECT**: Always the first message. Block until CONNACK received before sending anything else.

2. **PUBLISH QoS 0**: Most common for Home Assistant. Send and continue immediately.

3. **PUBLISH QoS 1**: For important messages (e.g., config updates). Wait for PUBACK with matching Packet ID.

4. **SUBSCRIBE**: Wait for SUBACK to confirm subscriptions were accepted before assuming you'll receive messages.

5. **PINGREQ**: Send when no other messages have been sent during keep-alive period. Wait for PINGRESP.

6. **Receiving PUBLISH**: When broker sends PUBLISH with QoS 1, you must respond with PUBACK containing the same Packet ID.

## Fixed Header Format

Every MQTT message starts with a 2+ byte fixed header:

```
Byte 1: Control Packet Type + Flags
┌────────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐
│  Bit   │  7  │  6  │  5  │  4  │  3  │  2  │  1  │  0  │
├────────┼─────┴─────┴─────┴─────┼─────┴─────┴─────┴─────┤
│ Field  │   Message Type (4)    │      Flags (4)        │
└────────┴───────────────────────┴───────────────────────┘

Byte 2+: Remaining Length (1-4 bytes, variable length encoding)
```

### Message Types
- `1` = CONNECT
- `2` = CONNACK
- `3` = PUBLISH
- `4` = PUBACK
- `8` = SUBSCRIBE
- `9` = SUBACK
- `10` = UNSUBSCRIBE
- `11` = UNSUBACK
- `12` = PINGREQ
- `13` = PINGRESP
- `14` = DISCONNECT

### Fixed Header Flags (Bits 0-3)

The meaning of the 4 flag bits depends on the message type.

#### PUBLISH Flags (Message Type 3)

For PUBLISH messages, all 4 flag bits are used:

```
Bit 3: DUP (Duplicate delivery)
Bit 2-1: QoS level (2 bits)
Bit 0: RETAIN
```

**Bit 3 - DUP (Duplicate)**:
- `0` = First delivery attempt
- `1` = Message is being re-delivered (only for QoS > 0)

**Bits 2-1 - QoS Level**:
- `00` (0) = At most once (fire and forget)
- `01` (1) = At least once (acknowledged)
- `10` (2) = Exactly once (four-way handshake)
- `11` (3) = Reserved (invalid)

**Bit 0 - RETAIN**:
- `0` = Normal message
- `1` = Broker should retain this message for new subscribers

**Examples**:
```
0x30 = 0011 0000 = PUBLISH, DUP=0, QoS=0, RETAIN=0
0x31 = 0011 0001 = PUBLISH, DUP=0, QoS=0, RETAIN=1 (retained message)
0x32 = 0011 0010 = PUBLISH, DUP=0, QoS=1, RETAIN=0
0x33 = 0011 0011 = PUBLISH, DUP=0, QoS=1, RETAIN=1
0x34 = 0011 0100 = PUBLISH, DUP=0, QoS=2, RETAIN=0
0x3A = 0011 1010 = PUBLISH, DUP=1, QoS=1, RETAIN=0 (re-delivery)
```

#### SUBSCRIBE Flags (Message Type 8)

For SUBSCRIBE, flags MUST be `0010` (bit 1 = 1):

```
Fixed header byte 1: 0x82 = 1000 0010
```

This is mandated by the MQTT specification. Other flag values are protocol violations.

#### UNSUBSCRIBE Flags (Message Type 10)

For UNSUBSCRIBE, flags MUST be `0010` (bit 1 = 1):

```
Fixed header byte 1: 0xA2 = 1010 0010
```

#### All Other Message Types

For all other message types (CONNECT, CONNACK, PUBACK, SUBACK, UNSUBACK, PINGREQ, PINGRESP, DISCONNECT), the flags MUST be `0000`:

```
CONNECT:    0x10 = 0001 0000
CONNACK:    0x20 = 0010 0000
PUBACK:     0x40 = 0100 0000
SUBACK:     0x90 = 1001 0000
UNSUBACK:   0xB0 = 1011 0000
PINGREQ:    0xC0 = 1100 0000
PINGRESP:   0xD0 = 1101 0000
DISCONNECT: 0xE0 = 1110 0000
```

Any other flag values for these message types are protocol violations.

### Summary Table

| Message Type | Required Flags | Flag Meaning |
|--------------|----------------|--------------|
| CONNECT (1) | `0000` | Reserved |
| CONNACK (2) | `0000` | Reserved |
| **PUBLISH (3)** | **`DQQR`** | **D=DUP, QQ=QoS, R=RETAIN** |
| PUBACK (4) | `0000` | Reserved |
| SUBSCRIBE (8) | `0010` | Fixed |
| SUBACK (9) | `0000` | Reserved |
| UNSUBSCRIBE (10) | `0010` | Fixed |
| UNSUBACK (11) | `0000` | Reserved |
| PINGREQ (12) | `0000` | Reserved |
| PINGRESP (13) | `0000` | Reserved |
| DISCONNECT (14) | `0000` | Reserved |

**Key Point**: Only PUBLISH messages use flags meaningfully. SUBSCRIBE and UNSUBSCRIBE have fixed flag values, and all other message types must have flags set to 0.

### Remaining Length Encoding

The remaining length uses a variable-length encoding (1-4 bytes):
- Each byte encodes 7 bits of data and 1 continuation bit
- Bit 7 = continuation bit (1 = more bytes follow, 0 = last byte)
- Bits 6-0 = length value

Example decoding:
```
0x7F          = 127 bytes
0x80 0x01     = 128 bytes
0xFF 0x7F     = 16,383 bytes
0xFF 0xFF 0x7F = 2,097,151 bytes
```

## Data Types

### UTF-8 String

**Important**: String length is a **fixed 2-byte unsigned integer (u16)**, NOT a variable-length integer like the Remaining Length field.

#### Format
```
┌──────────────┬──────────────────┬─────────────────────────┐
│ Length MSB   │ Length LSB       │ UTF-8 Encoded String    │
│ (byte 0)     │ (byte 1)         │ (N bytes)               │
└──────────────┴──────────────────┴─────────────────────────┘
      ↑──── u16 big-endian ────↑

Total size: 2 bytes + string length
Maximum string length: 65,535 bytes (0xFFFF)
```

#### Encoding Details
- **Length prefix**: 2 bytes, unsigned 16-bit integer
- **Byte order**: Big-endian (MSB first, then LSB)
- **String encoding**: UTF-8
- **Maximum length**: 65,535 bytes (not 65,536 - zero-length strings are allowed)
- This is **different** from the variable-length encoding used for "Remaining Length"

#### Examples

**Example 1: "MQTT" (4 bytes)**
```
0x00 0x04 'M' 'Q' 'T' 'T'

Breakdown:
0x00 0x04 = length 4 (u16 big-endian)
'M' 'Q' 'T' 'T' = 0x4D 0x51 0x54 0x54
```

**Example 2: "ha-client" (9 bytes)**
```
0x00 0x09 'h' 'a' '-' 'c' 'l' 'i' 'e' 'n' 't'

Breakdown:
0x00 0x09 = length 9
```

**Example 3: "homeassistant/sensor/temperature/state" (39 bytes)**
```
0x00 0x27 'h' 'o' 'm' 'e' 'a' 's' 's' 'i' 's' 't' ...

Breakdown:
0x00 0x27 = length 39 (0x27 = 39 in decimal)
```

**Example 4: Empty string (0 bytes)**
```
0x00 0x00

Breakdown:
0x00 0x00 = length 0 (valid in MQTT)
```

**Example 5: Long string (300 bytes)**
```
0x01 0x2C ...300 bytes of UTF-8 data...

Breakdown:
0x01 0x2C = length 300 (256 + 44 = 300)
```

#### Parsing String in Rust
```rust
fn parse_mqtt_string(buf: &[u8]) -> Result<(&str, &[u8])> {
    // Read u16 big-endian length
    if buf.len() < 2 {
        return Err(Error::InsufficientData);
    }
    let len = u16::from_be_bytes([buf[0], buf[1]]) as usize;

    // Read string data
    if buf.len() < 2 + len {
        return Err(Error::InsufficientData);
    }
    let string_bytes = &buf[2..2 + len];
    let string = core::str::from_utf8(string_bytes)?;
    let remaining = &buf[2 + len..];

    Ok((string, remaining))
}
```

#### Writing String in Rust
```rust
fn write_mqtt_string(buf: &mut [u8], s: &str) -> Result<usize> {
    let len = s.len();
    if len > 65535 {
        return Err(Error::StringTooLong);
    }
    if buf.len() < 2 + len {
        return Err(Error::InsufficientBuffer);
    }

    // Write u16 big-endian length
    buf[0] = (len >> 8) as u8;  // MSB
    buf[1] = (len & 0xFF) as u8; // LSB

    // Write string bytes
    buf[2..2 + len].copy_from_slice(s.as_bytes());

    Ok(2 + len)
}
```

### Binary Data
Same format as UTF-8 string (2-byte u16 length prefix + data), but contains raw binary data instead of UTF-8 text. Used for passwords and message payloads.

## Building MQTT Messages: Handling the Length Challenge

### The Problem

The fixed header contains the "Remaining Length" field which specifies the number of bytes following the fixed header. But you need to write the fixed header first, before you know how long the rest of the message will be.

### Solution Approaches

#### Approach 1: Calculate Length First (Recommended for Embedded)

Calculate the message size before writing anything. This avoids buffering the entire message.

```rust
// Step 1: Calculate sizes
let client_id = "ha-sensor";
let topic = "homeassistant/status";

let client_id_len = 2 + client_id.len();  // u16 prefix + string
let topic_len = 2 + topic.len();

// For CONNECT: protocol name + level + flags + keep-alive + payload
let remaining_length =
    (2 + 4) +           // Protocol name "MQTT" (2 byte len + 4 bytes)
    1 +                 // Protocol level
    1 +                 // Connect flags
    2 +                 // Keep-alive
    client_id_len;      // Client ID

// Step 2: Write fixed header with calculated length
let control_byte = 0x10;  // CONNECT
write_u8(buf, control_byte);
write_varint(buf, remaining_length);

// Step 3: Write variable header and payload
write_mqtt_string(buf, "MQTT");
write_u8(buf, 0x04);  // Protocol level
write_u8(buf, 0x02);  // Connect flags
write_u16_be(buf, 60); // Keep-alive
write_mqtt_string(buf, client_id);
```

**Pros**:
- No buffering needed (good for embedded)
- Direct write to TCP socket
- Minimal memory usage

**Cons**:
- Must calculate sizes carefully
- Easy to make mistakes

#### Approach 2: Reserve Space and Backfill

Reserve maximum space for the header (5 bytes), write the message, then go back and fill in the actual length.

```rust
fn build_publish(buf: &mut [u8], topic: &str, payload: &[u8]) -> usize {
    let mut pos = 0;

    // Step 1: Reserve space for fixed header (max 5 bytes)
    let header_start = pos;
    pos += 5;  // Maximum: 1 byte control + 4 bytes remaining length

    // Step 2: Write variable header and payload
    let payload_start = pos;
    pos += write_mqtt_string(&mut buf[pos..], topic);
    buf[pos..pos + payload.len()].copy_from_slice(payload);
    pos += payload.len();

    // Step 3: Calculate actual remaining length
    let remaining_length = pos - payload_start;

    // Step 4: Encode remaining length to temp buffer
    let mut temp = [0u8; 4];
    let varint_len = encode_varint(remaining_length as u32, &mut temp);

    // Step 5: Backfill header (shift if needed)
    let actual_header_len = 1 + varint_len;
    let shift = 5 - actual_header_len;
    if shift > 0 {
        // Shift payload left to remove unused header bytes
        buf.copy_within(payload_start..pos, payload_start - shift);
        pos -= shift;
    }

    // Step 6: Write actual header
    buf[0] = 0x30;  // PUBLISH QoS 0
    buf[1..1 + varint_len].copy_from_slice(&temp[..varint_len]);

    pos
}
```

**Pros**:
- Single pass through data
- No size calculation needed upfront

**Cons**:
- May need to shift data (costly on embedded)
- Wastes up to 4 bytes initially
- More complex

#### Approach 3: Two-Buffer Strategy

Write payload to one buffer, then construct final message.

```rust
fn build_connect(out: &mut [u8]) -> usize {
    let mut payload_buf = [0u8; 256];
    let mut payload_pos = 0;

    // Step 1: Build variable header + payload in temp buffer
    payload_pos += write_mqtt_string(&mut payload_buf[payload_pos..], "MQTT");
    payload_buf[payload_pos] = 0x04; payload_pos += 1;  // Protocol level
    payload_buf[payload_pos] = 0x02; payload_pos += 1;  // Connect flags
    payload_pos += write_u16_be(&mut payload_buf[payload_pos..], 60); // Keep-alive
    payload_pos += write_mqtt_string(&mut payload_buf[payload_pos..], "ha-client");

    let remaining_length = payload_pos;

    // Step 2: Write fixed header to output
    let mut pos = 0;
    out[pos] = 0x10; pos += 1;  // CONNECT
    pos += write_varint(&mut out[pos..], remaining_length as u32);

    // Step 3: Copy payload to output
    out[pos..pos + payload_pos].copy_from_slice(&payload_buf[..payload_pos]);
    pos + payload_pos
}
```

**Pros**:
- Simple and clear
- No backfilling needed

**Cons**:
- Uses 2x memory (bad for embedded)
- Extra copy operation

#### Approach 4: Builder Pattern with Deferred Write

Build a message description, calculate size, then serialize.

```rust
struct ConnectMessage<'a> {
    client_id: &'a str,
    clean_session: bool,
    keep_alive: u16,
}

impl<'a> ConnectMessage<'a> {
    fn calculate_size(&self) -> usize {
        let mut size = 0;
        size += 2 + 4;  // "MQTT"
        size += 1;      // Protocol level
        size += 1;      // Connect flags
        size += 2;      // Keep-alive
        size += 2 + self.client_id.len();  // Client ID
        size
    }

    fn serialize(&self, buf: &mut [u8]) -> usize {
        let remaining_length = self.calculate_size();
        let mut pos = 0;

        // Write fixed header
        buf[pos] = 0x10; pos += 1;
        pos += write_varint(&mut buf[pos..], remaining_length as u32);

        // Write variable header + payload
        pos += write_mqtt_string(&mut buf[pos..], "MQTT");
        buf[pos] = 0x04; pos += 1;
        let flags = if self.clean_session { 0x02 } else { 0x00 };
        buf[pos] = flags; pos += 1;
        pos += write_u16_be(&mut buf[pos..], self.keep_alive);
        pos += write_mqtt_string(&mut buf[pos..], self.client_id);

        pos
    }
}
```

**Pros**:
- Clean API
- Size calculation is explicit and testable
- No wasted space or copying

**Cons**:
- More code
- Need to keep calculation in sync with serialization

### Recommended Approach for Embedded Systems

For embedded systems like RP2350 with Embassy:

**Use Approach 1 (Calculate First) or Approach 4 (Builder Pattern)**

```rust
// Example: Helper to calculate MQTT string size
fn mqtt_string_size(s: &str) -> usize {
    2 + s.len()
}

// Example: Build PUBLISH message
fn build_publish_qos0(
    buf: &mut [u8],
    topic: &str,
    payload: &[u8]
) -> usize {
    // Calculate remaining length
    let remaining_length =
        mqtt_string_size(topic) +  // Topic
        payload.len();              // Payload

    let mut pos = 0;

    // Write fixed header
    buf[pos] = 0x30; pos += 1;  // PUBLISH QoS 0
    pos += write_varint(&mut buf[pos..], remaining_length as u32);

    // Write variable header
    pos += write_mqtt_string(&mut buf[pos..], topic);

    // Write payload
    buf[pos..pos + payload.len()].copy_from_slice(payload);
    pos += payload.len();

    pos
}
```

### Key Tips

1. **Create size calculation helpers**: Make functions like `mqtt_string_size()`, `connect_size()` to avoid mistakes

2. **Maximum varint size**: The remaining length varint can be 1-4 bytes. For small messages (<128 bytes), it's always 1 byte. This is predictable.

3. **QoS 0 PUBLISH is simplest**: No packet identifier needed, making size calculation trivial

4. **Test your calculations**: Write unit tests comparing calculated size vs actual serialized size

5. **Consider const generics**: For fixed message types, you can calculate sizes at compile time

## CONNECT Message

Client initiates connection to broker.

### Fixed Header
```
Byte 1: 0x10 (Message Type = 1, Flags = 0)
Byte 2+: Remaining Length
```

### Variable Header
```
┌─────────────────────────────────────┐
│ Protocol Name (UTF-8 String)        │
│ "MQTT" (0x00 0x04 0x4D 0x51 0x54 0x54) for MQTT 3.1.1
├─────────────────────────────────────┤
│ Protocol Level (1 byte)             │
│ 0x04 for MQTT 3.1.1                 │
│ 0x05 for MQTT 5.0                   │
├─────────────────────────────────────┤
│ Connect Flags (1 byte)              │
│ Bit 7: User Name Flag               │
│ Bit 6: Password Flag                │
│ Bit 5: Will Retain                  │
│ Bit 4-3: Will QoS (2 bits)          │
│ Bit 2: Will Flag                    │
│ Bit 1: Clean Session (v3.1.1)       │
│       Clean Start (v5.0)            │
│ Bit 0: Reserved (must be 0)         │
├─────────────────────────────────────┤
│ Keep Alive (2 bytes, MSB first)     │
│ Seconds, 0 = disabled               │
└─────────────────────────────────────┘
```

### Payload (in order)
1. **Client ID** (UTF-8 String) - Required
2. **Will Topic** (UTF-8 String) - If Will Flag = 1
3. **Will Payload** (Binary Data) - If Will Flag = 1
4. **User Name** (UTF-8 String) - If User Name Flag = 1
5. **Password** (Binary Data) - If Password Flag = 1


#### Connect Flags Breakdown

The Connect Flags byte (byte 10 of CONNECT message) controls authentication and the Last Will and Testament.

**Last Will and Testament (LWT)**: A message the broker will automatically publish if the client disconnects unexpectedly (network failure, crash, etc.). This is useful for Home Assistant to detect when a device goes offline.

**Connect Flags bit layout:**
```
Bit:     7      6      5        4    3      2         1              0
      ┌──────┬──────┬──────┬──────┬──────┬──────┬──────────────┬──────┐
      │ User │ Pass │ Will │   Will QoS  │ Will │   Clean      │ Res. │
      │ Name │ word │Retain│             │ Flag │   Session    │ (0)  │
      └──────┴──────┴──────┴──────┴──────┴──────┴──────────────┴──────┘
```

**Bits 4-3: Will QoS** (only applies if Bit 2 Will Flag = 1):
- `00` (0) = QoS 0 for Will message (fire and forget)
- `01` (1) = QoS 1 for Will message (acknowledged)
- `10` (2) = QoS 2 for Will message (exactly once)
- `11` (3) = Invalid, must not be used

**Important**: If Will Flag (bit 2) = 0, then Will QoS MUST be set to 00 and Will Retain (bit 5) MUST be 0.

#### Connect Flags Examples

**Example 1: Minimal connection (no Will, no auth)**
```
Binary:  0000 0010
Hex:     0x02
- User Name Flag: 0 (no username)
- Password Flag: 0 (no password)
- Will Retain: 0
- Will QoS: 00 (not used, Will Flag = 0)
- Will Flag: 0 (no Will message)
- Clean Session: 1
- Reserved: 0
```

**Example 2: With Last Will (QoS 0, no retain)**
```
Binary:  0000 0110
Hex:     0x06
- User Name Flag: 0
- Password Flag: 0
- Will Retain: 0 (don't retain Will message)
- Will QoS: 00 (QoS 0 for Will)
- Will Flag: 1 (Will message enabled)
- Clean Session: 1
- Reserved: 0

Must include Will Topic and Will Payload in CONNECT payload.
```

**Example 3: With Last Will (QoS 1, retain)**
```
Binary:  0011 0110
Hex:     0x36
- User Name Flag: 0
- Password Flag: 0
- Will Retain: 1 (broker retains Will message)
- Will QoS: 01 (QoS 1 for Will)
- Will Flag: 1 (Will message enabled)
- Clean Session: 1
- Reserved: 0

Will message will be retained and delivered with QoS 1.
```

**Example 4: With authentication and Last Will**
```
Binary:  1100 0110
Hex:     0xC6
- User Name Flag: 1 (username provided)
- Password Flag: 1 (password provided)
- Will Retain: 0
- Will QoS: 00 (QoS 0 for Will)
- Will Flag: 1 (Will message enabled)
- Clean Session: 1
- Reserved: 0

Must include Username, Password, Will Topic, and Will Payload in payload.
```

### Example CONNECT (Simple)
```
Client ID: "ha-client"
Clean Session: true
Keep Alive: 60 seconds
No Will, No Auth

0x10                    // Fixed header: CONNECT
0x17                    // Remaining length: 23 bytes
0x00 0x04 'M' 'Q' 'T' 'T'  // Protocol name
0x04                    // Protocol level: 3.1.1
0x02                    // Connect flags: 0000 0010 = Clean Session only
0x00 0x3C               // Keep Alive: 60 seconds
0x00 0x09 'h' 'a' '-' 'c' 'l' 'i' 'e' 'n' 't'  // Client ID: "ha-client"
```

### Example CONNECT (With Last Will)
```
Client ID: "sensor1"
Clean Session: true
Keep Alive: 60 seconds
Will Topic: "homeassistant/sensor1/availability"
Will Payload: "offline"
Will QoS: 0
Will Retain: true

0x10                    // Fixed header: CONNECT
0x3C                    // Remaining length: 60 bytes
0x00 0x04 'M' 'Q' 'T' 'T'  // Protocol name: "MQTT"
0x04                    // Protocol level: 3.1.1
0x26                    // Connect flags: 0010 0110
                        //   Bit 5: Will Retain = 1
                        //   Bit 4-3: Will QoS = 00
                        //   Bit 2: Will Flag = 1
                        //   Bit 1: Clean Session = 1
0x00 0x3C               // Keep Alive: 60 seconds
0x00 0x07 's' 'e' 'n' 's' 'o' 'r' '1'  // Client ID: "sensor1"
0x00 0x25               // Will Topic length: 37 bytes
'h' 'o' 'm' 'e' 'a' 's' 's' 'i' 's' 't' 'a' 'n' 't' '/'
's' 'e' 'n' 's' 'o' 'r' '1' '/'
'a' 'v' 'a' 'i' 'l' 'a' 'b' 'i' 'l' 'i' 't' 'y'
0x00 0x07               // Will Payload length: 7 bytes
'o' 'f' 'f' 'l' 'i' 'n' 'e'  // Will Payload: "offline"
```

When this client disconnects unexpectedly, the broker will publish:
- Topic: `homeassistant/sensor1/availability`
- Payload: `offline`
- QoS: 0
- Retained: Yes (so Home Assistant sees it even if it reconnects later)

## DISCONNECT Message

Client graceful disconnect (MQTT 3.1.1).

### Fixed Header
```
Byte 1: 0xE0 (Message Type = 14, Flags = 0)
Byte 2: 0x00 (Remaining Length = 0)
```

No Variable Header or Payload in MQTT 3.1.1.

### Complete Message
```
0xE0 0x00
```

## PUBLISH Message

Publish message to a topic.

### Fixed Header
```
Byte 1: 0x3X (Message Type = 3, Flags in bits 0-3)
  Bit 3: DUP flag (duplicate delivery)
  Bit 2-1: QoS level (00, 01, or 10)
  Bit 0: RETAIN flag

Examples:
  0x30 = PUBLISH, QoS 0, no retain
  0x31 = PUBLISH, QoS 0, retain
  0x32 = PUBLISH, QoS 1, no retain
  0x34 = PUBLISH, QoS 2, no retain

Byte 2+: Remaining Length
```

### Variable Header
```
┌─────────────────────────────────────┐
│ Topic Name (UTF-8 String)           │
├─────────────────────────────────────┤
│ Packet Identifier (2 bytes)         │
│ Only present if QoS > 0             │
└─────────────────────────────────────┘
```

### Payload
Raw message data (can be text or binary).

### Example PUBLISH
```
Topic: "homeassistant/sensor/temp/state"
Payload: "23.5"
QoS: 0
Retain: false

0x30                    // Fixed header: PUBLISH, QoS 0
0x29                    // Remaining length: 41 bytes
0x00 0x23               // Topic length: 35 bytes
'h' 'o' 'm' 'e' 'a' 's' 's' 'i' 's' 't' 'a' 'n' 't' '/'
's' 'e' 'n' 's' 'o' 'r' '/' 't' 'e' 'm' 'p' '/'
's' 't' 'a' 't' 'e'
'2' '3' '.' '5'        // Payload: "23.5"
```

### Example PUBLISH (QoS 1)
```
Topic: "homeassistant/switch/state"
Payload: "ON"
QoS: 1
Packet ID: 1

0x32                    // Fixed header: PUBLISH, QoS 1
0x1F                    // Remaining length
0x00 0x1B               // Topic length: 27 bytes
'h' 'o' 'm' 'e' 'a' 's' 's' 'i' 's' 't' 'a' 'n' 't' '/'
's' 'w' 'i' 't' 'c' 'h' '/' 's' 't' 'a' 't' 'e'
0x00 0x01               // Packet Identifier: 1
'O' 'N'                 // Payload
```

## SUBSCRIBE Message

Subscribe to one or more topics.

### Fixed Header
```
Byte 1: 0x82 (Message Type = 8, Flags = 0010)
  Bit 1 MUST be 1 for SUBSCRIBE
Byte 2+: Remaining Length
```

### Variable Header
```
┌─────────────────────────────────────┐
│ Packet Identifier (2 bytes, MSB)    │
└─────────────────────────────────────┘
```

### Payload
List of topic filters with QoS levels:
```
For each subscription:
┌─────────────────────────────────────┐
│ Topic Filter (UTF-8 String)         │
│ Supports wildcards:                 │
│   + = single level wildcard         │
│   # = multi level wildcard          │
├─────────────────────────────────────┤
│ QoS (1 byte)                        │
│ Bits 7-2: Reserved (must be 0)      │
│ Bits 1-0: QoS level (0, 1, or 2)    │
└─────────────────────────────────────┘
```

### Example SUBSCRIBE
```
Subscribe to: "homeassistant/#" (all topics under homeassistant/)
QoS: 0
Packet ID: 2

0x82                    // Fixed header: SUBSCRIBE
0x13                    // Remaining length: 19 bytes
0x00 0x02               // Packet Identifier: 2
0x00 0x0F               // Topic length: 15 bytes
'h' 'o' 'm' 'e' 'a' 's' 's' 'i' 's' 't' 'a' 'n' 't' '/' '#'
0x00                    // QoS: 0
```

### Example SUBSCRIBE (Multiple Topics)
```
Subscribe to:
  1. "homeassistant/status" QoS 0
  2. "homeassistant/+/state" QoS 1
Packet ID: 3

0x82                    // Fixed header
0x37                    // Remaining length
0x00 0x03               // Packet Identifier: 3
0x00 0x16               // Topic 1 length: 22 bytes
'h' 'o' 'm' 'e' 'a' 's' 's' 'i' 's' 't' 'a' 'n' 't' '/'
's' 't' 'a' 't' 'u' 's'
0x00                    // QoS: 0
0x00 0x16               // Topic 2 length: 22 bytes
'h' 'o' 'm' 'e' 'a' 's' 's' 'i' 's' 't' 'a' 'n' 't' '/'
'+' '/' 's' 't' 'a' 't' 'e'
0x01                    // QoS: 1
```

## QoS Levels

Home Assistant typically uses:
- **QoS 0**: At most once delivery (fire and forget)
- **QoS 1**: At least once delivery (requires PUBACK)
- **QoS 2**: Exactly once delivery (requires PUBREC, PUBREL, PUBCOMP)

For most Home Assistant integrations, QoS 0 is sufficient. Use QoS 1 for important state changes.

## Topic Wildcards

- `+` (single-level): Matches one topic level
  - `homeassistant/+/state` matches `homeassistant/sensor/state` and `homeassistant/switch/state`
- `#` (multi-level): Matches zero or more levels, must be last character
  - `homeassistant/#` matches everything under `homeassistant/`

## Home Assistant Topic Structure

Common Home Assistant MQTT topics:
```
homeassistant/<component>/<node_id>/<object_id>/config
homeassistant/<component>/<node_id>/<object_id>/state
homeassistant/<component>/<node_id>/<object_id>/command
homeassistant/status
```

Example:
```
homeassistant/sensor/living_room/temperature/config
homeassistant/sensor/living_room/temperature/state
homeassistant/switch/bedroom/light/config
homeassistant/switch/bedroom/light/state
homeassistant/switch/bedroom/light/command
```

## Typical Flow for Home Assistant Device

1. **Connect** to broker
2. **Subscribe** to:
   - `homeassistant/status` (to detect HA restarts)
   - Command topics for your devices (e.g., `homeassistant/switch/+/command`)
3. **Publish** discovery configs to `homeassistant/.../config` with retain=true
4. **Publish** state updates to `homeassistant/.../state`
5. **Receive** commands on subscribed topics
6. On disconnect, broker sends Will message if configured

## Keep-Alive and PINGREQ/PINGRESP

If no messages are sent within keep-alive period:
- Client sends PINGREQ: `0xC0 0x00`
- Broker responds with PINGRESP: `0xD0 0x00`

If broker doesn't receive any message within 1.5x keep-alive, it disconnects the client.

## Broker Response Messages

These are messages the broker sends back to the client that you need to parse.

### CONNACK (Connection Acknowledgment)

Broker response to CONNECT.

#### Fixed Header
```
Byte 1: 0x20 (Message Type = 2, Flags = 0)
Byte 2: 0x02 (Remaining Length = 2)
```

#### Variable Header
```
┌─────────────────────────────────────┐
│ Connect Acknowledge Flags (1 byte)  │
│ Bit 0: Session Present              │
│ Bits 7-1: Reserved (must be 0)      │
├─────────────────────────────────────┤
│ Connect Return Code (1 byte)        │
│ 0x00 = Connection Accepted          │
│ 0x01 = Unacceptable protocol ver    │
│ 0x02 = Identifier rejected          │
│ 0x03 = Server unavailable           │
│ 0x04 = Bad username/password        │
│ 0x05 = Not authorized               │
└─────────────────────────────────────┘
```

#### Example CONNACK (Success)
```
0x20 0x02    // Fixed header
0x00         // Session Present = 0
0x00         // Return code = 0 (accepted)
```

#### Example CONNACK (Connection Refused)
```
0x20 0x02    // Fixed header
0x00         // Session Present = 0
0x05         // Return code = 5 (not authorized)
```

**Implementation**: Check return code is 0x00 before proceeding. If non-zero, connection failed.

---

### PUBACK (Publish Acknowledgment)

Broker response to PUBLISH with QoS 1.

#### Fixed Header
```
Byte 1: 0x40 (Message Type = 4, Flags = 0)
Byte 2: 0x02 (Remaining Length = 2)
```

#### Variable Header
```
┌─────────────────────────────────────┐
│ Packet Identifier (2 bytes, MSB)    │
│ Must match the PUBLISH packet ID    │
└─────────────────────────────────────┘
```

#### Example PUBACK
```
For PUBLISH with Packet ID = 42 (0x002A):

0x40 0x02    // Fixed header
0x00 0x2A    // Packet Identifier: 42
```

**Implementation**: Match Packet ID with the PUBLISH you sent. Once received, the message is acknowledged.

---

### SUBACK (Subscribe Acknowledgment)

Broker response to SUBSCRIBE.

#### Fixed Header
```
Byte 1: 0x90 (Message Type = 9, Flags = 0)
Byte 2+: Remaining Length
```

#### Variable Header
```
┌─────────────────────────────────────┐
│ Packet Identifier (2 bytes, MSB)    │
│ Must match the SUBSCRIBE packet ID  │
└─────────────────────────────────────┘
```

#### Payload
One return code for each topic filter in the SUBSCRIBE request:
```
┌─────────────────────────────────────┐
│ Return Code (1 byte per topic)      │
│ 0x00 = Success, QoS 0               │
│ 0x01 = Success, QoS 1               │
│ 0x02 = Success, QoS 2               │
│ 0x80 = Failure                      │
└─────────────────────────────────────┘
```

#### Example SUBACK (Single Topic)
```
For SUBSCRIBE with Packet ID = 2, one topic requesting QoS 0:

0x90 0x03    // Fixed header, remaining = 3 bytes
0x00 0x02    // Packet Identifier: 2
0x00         // Return code: Success, QoS 0
```

#### Example SUBACK (Multiple Topics)
```
For SUBSCRIBE with Packet ID = 3, two topics:

0x90 0x04    // Fixed header, remaining = 4 bytes
0x00 0x03    // Packet Identifier: 3
0x00         // Topic 1: Success, QoS 0
0x01         // Topic 2: Success, QoS 1
```

#### Example SUBACK (Failed Subscription)
```
0x90 0x03    // Fixed header
0x00 0x04    // Packet Identifier: 4
0x80         // Return code: Failure
```

**Implementation**: Check each return code. 0x80 means that topic subscription failed.

---

### UNSUBACK (Unsubscribe Acknowledgment)

Broker response to UNSUBSCRIBE.

#### Fixed Header
```
Byte 1: 0xB0 (Message Type = 11, Flags = 0)
Byte 2: 0x02 (Remaining Length = 2)
```

#### Variable Header
```
┌─────────────────────────────────────┐
│ Packet Identifier (2 bytes, MSB)    │
│ Must match UNSUBSCRIBE packet ID    │
└─────────────────────────────────────┘
```

#### Example UNSUBACK
```
For UNSUBSCRIBE with Packet ID = 5:

0xB0 0x02    // Fixed header
0x00 0x05    // Packet Identifier: 5
```

**Implementation**: Once received, unsubscription is confirmed.
