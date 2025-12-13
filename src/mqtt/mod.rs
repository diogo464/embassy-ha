#![allow(unused)]

mod connect_code;
mod field;
mod packet_id;
mod protocol;
mod qos;
mod rx;
mod transport;
mod tx;
mod varint;

pub use connect_code::ConnectCode;
use embedded_io_async::ReadExactError;
pub use packet_id::PacketId;
pub use qos::Qos;
pub use transport::Transport;

use self::{field::FieldBuffer, transport::TransportExt as _};

const DEFAULT_CLIENT_RX_BUFFER_SIZE: usize = 512;
const DEFAULT_CLIENT_TX_BUFFER_SIZE: usize = 512;

pub enum Error<T: Transport> {
    Transport(T::Error),
    TransportEOF,
    InsufficientBufferSpace,
    Protocol(&'static str),
    ConnectFailed(ConnectCode),
}

impl<T: Transport> core::fmt::Debug for Error<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Transport(err) => f.debug_tuple("Transport").field(err).finish(),
            Error::TransportEOF => f.write_str("TransportEOF"),
            Error::InsufficientBufferSpace => f.write_str("InsufficientBufferSpace"),
            Error::Protocol(msg) => f.debug_tuple("ProtocolError").field(msg).finish(),
            Error::ConnectFailed(code) => f.debug_tuple("ConnectFailed").field(code).finish(),
        }
    }
}

impl<T: Transport> core::fmt::Display for Error<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Transport(err) => write!(f, "transport error: {:?}", err),
            Error::TransportEOF => write!(f, "unexpected end of transport stream"),
            Error::InsufficientBufferSpace => {
                write!(f, "insufficient buffer space to receive packet")
            }
            Error::Protocol(msg) => write!(f, "MQTT protocol error: {}", msg),
            Error::ConnectFailed(code) => write!(f, "connection failed: {}", code),
        }
    }
}

impl<T: Transport> core::error::Error for Error<T>
where
    T::Error: core::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::Transport(err) => Some(err),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct ConnectParams<'a> {
    pub will_topic: Option<&'a str>,
    pub will_payload: Option<&'a [u8]>,
    pub will_retain: bool,
    pub username: Option<&'a str>,
    pub password: Option<&'a [u8]>,
    pub keepalive: Option<u16>,
}

#[derive(Debug, Default)]
pub struct PublishParams {
    pub qos: Qos,
    pub retain: bool,
}

#[derive(Debug)]
pub enum PublishData<'a> {
    Inline(&'a [u8]),
    Deferred(usize),
}

#[derive(Debug)]
pub struct Publish<'a> {
    pub topic: &'a str,
    pub packet_id: Option<PacketId>,
    pub qos: Qos,
    pub retain: bool,
    pub data_len: usize,
}

#[derive(Debug)]
pub struct PublishAck {
    pub packet_id: PacketId,
}

#[derive(Debug)]
pub struct SubscribeAck {
    pub packet_id: PacketId,
    pub success: bool,
}

#[derive(Debug)]
pub struct UnsubscribeAck {
    pub packet_id: PacketId,
}

#[derive(Debug)]
pub enum Packet<'a> {
    Publish(Publish<'a>),
    PublishAck(PublishAck),
    SubscribeAck(SubscribeAck),
    UnsubscribeAck(UnsubscribeAck),
    PingResponse,
}

pub struct ClientResources<
    const RX: usize = DEFAULT_CLIENT_RX_BUFFER_SIZE,
    const TX: usize = DEFAULT_CLIENT_TX_BUFFER_SIZE,
> {
    rx_buffer: [u8; RX],
    tx_buffer: [u8; TX],
}

impl<const RX: usize, const TX: usize> Default for ClientResources<RX, TX> {
    fn default() -> Self {
        Self {
            rx_buffer: [0u8; RX],
            tx_buffer: [0u8; TX],
        }
    }
}

pub struct Client<'a, T> {
    transport: T,
    rx_buffer: &'a mut [u8],
    rx_buffer_len: usize,
    rx_buffer_skip: usize,
    rx_buffer_data: usize,
    tx_buffer: &'a mut [u8],
    next_packet_id: u16,
}

impl<'a, T> Client<'a, T> {
    pub fn new<const RX: usize, const TX: usize>(
        resources: &'a mut ClientResources<RX, TX>,
        transport: T,
    ) -> Self {
        Self {
            transport,
            rx_buffer: &mut resources.rx_buffer,
            rx_buffer_len: 0,
            rx_buffer_skip: 0,
            rx_buffer_data: 0,
            tx_buffer: &mut resources.tx_buffer,
            next_packet_id: 1,
        }
    }
}

impl<'a, T> Client<'a, T>
where
    T: Transport,
{
    fn allocate_packet_id(&mut self) -> PacketId {
        let packet_id = self.next_packet_id;
        self.next_packet_id = self.next_packet_id.wrapping_add(1);
        if self.next_packet_id == 0 {
            self.next_packet_id = 1;
        }
        PacketId::from(packet_id)
    }

    pub async fn connect(&mut self, client_id: &str) -> Result<(), Error<T>> {
        self.connect_with(client_id, Default::default()).await
    }

    pub async fn connect_with(
        &mut self,
        client_id: &str,
        params: ConnectParams<'_>,
    ) -> Result<(), Error<T>> {
        let mut buffer = FieldBuffer::default();
        tx::connect(
            &mut buffer,
            tx::Connect {
                client_id,
                clean_session: true,
                username: params.username,
                password: params.password,
                will_topic: params.will_topic,
                will_payload: params.will_payload,
                will_retain: params.will_retain,
                keepalive: params.keepalive,
            },
        );
        self.transport
            .write_fields(&buffer)
            .await
            .map_err(Error::Transport)?;
        self.transport.flush().await.map_err(Error::Transport)?;

        // Wait for CONNACK response
        match self.receive_inner().await? {
            rx::Packet::ConnAck {
                session_present: _,
                code,
            } => {
                if code == ConnectCode::ConnectionAccepted {
                    Ok(())
                } else {
                    Err(Error::ConnectFailed(code))
                }
            }
            _ => Err(Error::Protocol("expected CONNACK packet after CONNECT")),
        }
    }

    pub async fn ping(&mut self) -> Result<(), Error<T>> {
        let mut buffer = FieldBuffer::default();
        tx::pingreq(&mut buffer);

        self.transport
            .write_fields(&buffer)
            .await
            .map_err(Error::Transport)?;
        self.transport.flush().await.map_err(Error::Transport)?;

        Ok(())
    }

    pub async fn publish(&mut self, topic: &str, data: &[u8]) -> Result<PacketId, Error<T>> {
        self.publish_with(topic, data, Default::default()).await
    }

    pub async fn publish_with(
        &mut self,
        topic: &str,
        data: &[u8],
        params: PublishParams,
    ) -> Result<PacketId, Error<T>> {
        let packet_id = if params.qos.to_u8() > 0 {
            Some(self.allocate_packet_id())
        } else {
            None
        };

        let mut buffer = FieldBuffer::default();
        tx::publish(
            &mut buffer,
            tx::Publish {
                topic,
                payload: data,
                qos: params.qos,
                retain: params.retain,
                dup: false,
                packet_id,
            },
        );

        self.transport
            .write_fields(&buffer)
            .await
            .map_err(Error::Transport)?;
        self.transport.flush().await.map_err(Error::Transport)?;

        Ok(packet_id.unwrap_or(PacketId::from(0)))
    }

    pub async fn publish_ack(&mut self, packet_id: PacketId, qos: Qos) -> Result<(), Error<T>> {
        let mut buffer = FieldBuffer::default();

        match qos {
            Qos::AtMostOnce => {
                // QoS 0: No acknowledgment needed
                return Ok(());
            }
            Qos::AtLeastOnce => {
                // QoS 1: Send PUBACK
                tx::puback(&mut buffer, packet_id);
            }
            Qos::ExactlyOnce => todo!("not implemented"),
        }

        self.transport
            .write_fields(&buffer)
            .await
            .map_err(Error::Transport)?;
        self.transport.flush().await.map_err(Error::Transport)?;

        Ok(())
    }

    pub async fn subscribe(&mut self, topic: &str) -> Result<PacketId, Error<T>> {
        self.subscribe_with(topic, Qos::AtMostOnce).await
    }

    pub async fn subscribe_with(&mut self, topic: &str, qos: Qos) -> Result<PacketId, Error<T>> {
        let packet_id = self.allocate_packet_id();

        let mut buffer = FieldBuffer::default();
        tx::subscribe(
            &mut buffer,
            tx::Subscribe {
                topic,
                qos,
                packet_id,
            },
        );

        self.transport
            .write_fields(&buffer)
            .await
            .map_err(Error::Transport)?;
        self.transport.flush().await.map_err(Error::Transport)?;

        Ok(packet_id)
    }

    pub async fn unsubscribe(&mut self, topic: &str) -> Result<PacketId, Error<T>> {
        let packet_id = self.allocate_packet_id();

        let mut buffer = FieldBuffer::default();
        tx::unsubscribe(&mut buffer, tx::Unsubscribe { topic, packet_id });

        self.transport
            .write_fields(&buffer)
            .await
            .map_err(Error::Transport)?;
        self.transport.flush().await.map_err(Error::Transport)?;

        Ok(packet_id)
    }

    async fn receive_inner<'s>(&'s mut self) -> Result<rx::Packet<'s>, Error<T>> {
        self.skip_if_required();
        self.discard_data().await?;

        loop {
            let buf = &self.rx_buffer[..self.rx_buffer_len];
            match rx::decode(buf) {
                Ok(_) => {
                    // NOTE: stupid workaround for borrow checker, should not
                    // need to decode twice
                    let buf = &self.rx_buffer[..self.rx_buffer_len];
                    let (packet, n) = rx::decode(buf).unwrap();
                    self.rx_buffer_skip = n;
                    if let rx::Packet::Publish { data_len, .. } = &packet {
                        self.rx_buffer_data = *data_len;
                    }
                    return Ok(packet);
                }
                Err(err) => match err {
                    rx::Error::NeedMoreData => {
                        if self.rx_buffer.len() == self.rx_buffer_len {
                            return Err(Error::InsufficientBufferSpace);
                        }
                    }
                    rx::Error::InvalidPacket(msg) => return Err(Error::Protocol(msg)),
                    rx::Error::UnsupportedPacket { packet_type: _, .. } => {
                        return Err(Error::Protocol("unsupported packet type"));
                    }
                    rx::Error::UnknownPacket { packet_type: _, .. } => {
                        return Err(Error::Protocol("unknown packet type"));
                    }
                },
            }

            self.fill_rx_buffer().await?;
        }
    }

    pub async fn receive<'s>(&'s mut self) -> Result<Packet<'s>, Error<T>> {
        match self.receive_inner().await? {
            rx::Packet::ConnAck { .. } => Err(Error::Protocol("unexpected CONNACK packet")),
            rx::Packet::Publish {
                topic,
                packet_id,
                qos,
                retain,
                dup: _dup,
                data_len,
            } => Ok(Packet::Publish(Publish {
                topic,
                packet_id,
                qos,
                retain,
                data_len,
            })),
            rx::Packet::PubAck { packet_id } => Ok(Packet::PublishAck(PublishAck { packet_id })),
            rx::Packet::SubscribeAck { packet_id, success } => {
                Ok(Packet::SubscribeAck(SubscribeAck { packet_id, success }))
            }
            rx::Packet::UnsubscribeAck { packet_id } => {
                Ok(Packet::UnsubscribeAck(UnsubscribeAck { packet_id }))
            }
            rx::Packet::PingResp => Ok(Packet::PingResponse),
        }
    }

    pub async fn receive_data(&mut self, buf: &mut [u8]) -> Result<(), Error<T>> {
        self.skip_if_required();
        if buf.len() != self.rx_buffer_data {
            return Err(Error::InsufficientBufferSpace);
        }

        assert_eq!(self.rx_buffer_skip, 0);
        let from_buffer = self.rx_buffer_data.min(self.rx_buffer_len);
        let from_transport = self.rx_buffer_data.strict_sub(from_buffer);

        buf[..from_buffer].copy_from_slice(&self.rx_buffer[..from_buffer]);
        self.rx_buffer_len -= from_buffer;

        if from_transport > 0 {
            assert_eq!(self.rx_buffer_len, 0);
            self.transport
                .read_exact(&mut buf[from_buffer..])
                .await
                .map_err(|err| match err {
                    ReadExactError::UnexpectedEof => Error::<T>::TransportEOF,
                    ReadExactError::Other(e) => Error::Transport(e),
                })?;
        }
        self.rx_buffer_data = 0;

        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), Error<T>> {
        let mut buffer = FieldBuffer::default();
        tx::disconnect(&mut buffer);

        self.transport
            .write_fields(&buffer)
            .await
            .map_err(Error::Transport)?;
        self.transport.flush().await.map_err(Error::Transport)?;

        Ok(())
    }

    async fn fill_rx_buffer(&mut self) -> Result<(), Error<T>> {
        let n = self
            .transport
            .read(&mut self.rx_buffer[self.rx_buffer_len..])
            .await
            .map_err(Error::Transport)?;
        if n == 0 {
            return Err(Error::TransportEOF);
        }
        self.rx_buffer_len += n;

        Ok(())
    }

    fn skip_if_required(&mut self) {
        assert!(self.rx_buffer_len >= self.rx_buffer_skip);
        if self.rx_buffer_skip != 0 {
            self.rx_buffer.copy_within(self.rx_buffer_skip.., 0);
            self.rx_buffer_len = self.rx_buffer_len.strict_sub(self.rx_buffer_skip);
            self.rx_buffer_skip = 0;
        }
    }

    async fn discard_data(&mut self) -> Result<(), Error<T>> {
        if self.rx_buffer_data == 0 {
            return Ok(());
        }

        assert_eq!(self.rx_buffer_skip, 0);
        while self.rx_buffer_data > 0 {
            if self.rx_buffer_len <= self.rx_buffer_data {
                self.rx_buffer_data -= self.rx_buffer_len;
                self.rx_buffer_len = 0;
            } else {
                self.rx_buffer.copy_within(self.rx_buffer_data.., 0);
                self.rx_buffer_len -= self.rx_buffer_data;
                self.rx_buffer_data = 0;
            }
            self.fill_rx_buffer().await?;
        }

        Ok(())
    }
}
