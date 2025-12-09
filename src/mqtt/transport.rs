use super::{field::Field, varint};

pub trait Transport: embedded_io_async::Read + embedded_io_async::Write {}

impl<T> Transport for T where T: embedded_io_async::Read + embedded_io_async::Write {}

pub(crate) trait TransportExt: Transport {
    async fn write_fields(&mut self, fields: &[Field]) -> Result<(), Self::Error>;
}

impl<T> TransportExt for T
where
    T: Transport,
{
    async fn write_fields(&mut self, fields: &[Field<'_>]) -> Result<(), Self::Error> {
        for field in fields {
            match field {
                Field::U8(v) => self.write_all(&[*v]).await?,
                Field::U16(v) => self.write_all(&u16::to_be_bytes(*v)).await?,
                Field::VarInt(v) => {
                    let (v_buf, v_len) = varint::encode(*v);
                    self.write_all(&v_buf[..v_len]).await?;
                }
                Field::Buffer(v) => self.write_all(v).await?,
                Field::LenPrefixedBuffer(v) => {
                    self.write_all(&u16::to_be_bytes(u16::try_from(v.len()).unwrap()))
                        .await?;
                    self.write_all(v).await?;
                }
                Field::LenPrefixedString(v) => {
                    self.write_all(&u16::to_be_bytes(u16::try_from(v.len()).unwrap()))
                        .await?;
                    self.write_all(v.as_bytes()).await?;
                }
            }
        }
        Ok(())
    }
}
