#[derive(Debug)]
pub enum Error {
    NeedMoreData,
    InvalidVarInt,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::NeedMoreData => f.write_str("NeedMoreData"),
            Error::InvalidVarInt => f.write_str("InvalidVarInt"),
        }
    }
}

impl core::error::Error for Error {}

pub fn encode(mut v: u32) -> ([u8; 4], usize) {
    let mut encoded = [0u8; 4];
    let mut count = 0;

    loop {
        let mut byte = (v % 128) as u8;
        v /= 128;

        if v > 0 {
            byte |= 0x80; // Set continuation bit
        }

        encoded[count] = byte;
        count += 1;

        if v == 0 {
            break;
        }
    }

    (encoded, count)
}

pub fn decode(buf: &[u8]) -> Result<(u32, usize), Error> {
    let mut value = 0u32;

    let v = buf.get(0).ok_or(Error::NeedMoreData)?;
    value |= ((v & 0x7F) as u32) << 0;
    if v & 0x80 == 0 {
        return Ok((value, 1));
    }

    let v = buf.get(1).ok_or(Error::NeedMoreData)?;
    value |= ((v & 0x7F) as u32) << 7;
    if v & 0x80 == 0 {
        return Ok((value, 2));
    }

    let v = buf.get(2).ok_or(Error::NeedMoreData)?;
    value |= ((v & 0x7F) as u32) << 14;
    if v & 0x80 == 0 {
        return Ok((value, 3));
    }

    let v = buf.get(3).ok_or(Error::NeedMoreData)?;
    value |= ((v & 0x7F) as u32) << 21;
    if v & 0x80 != 0 {
        return Err(Error::InvalidVarInt);
    }

    Ok((value, 4))
}
