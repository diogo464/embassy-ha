use core::{
    convert::Infallible,
    future::{Future, pending},
    pin::pin,
    task::{Context, Poll, Waker},
};

use super::{Client, ClientResources, Packet};

// Two QoS 0 PUBLISH packets on topic "t" (ON, OFF), followed by PINGRESP.
const PACKETS: &[u8] = b"\x30\x05\x00\x01tON\x30\x06\x00\x01tOFF\xd0\x00";

struct BufferedTransport<'a> {
    chunks: &'a [&'a [u8]],
    offset: usize,
}

impl embedded_io_async::ErrorType for BufferedTransport<'_> {
    type Error = Infallible;
}

impl embedded_io_async::Read for BufferedTransport<'_> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        let Some(chunk) = self.chunks.first() else {
            // Model an open connection with no more bytes available. Reading
            // past a buffered packet must stall, rather than signal EOF.
            return pending().await;
        };
        let remaining = &chunk[self.offset..];
        let n = remaining.len().min(buf.len());
        buf[..n].copy_from_slice(&remaining[..n]);
        self.offset += n;
        if self.offset == chunk.len() {
            self.chunks = &self.chunks[1..];
            self.offset = 0;
        }
        Ok(n)
    }
}

impl embedded_io_async::Write for BufferedTransport<'_> {
    async fn write(&mut self, _: &[u8]) -> Result<usize, Self::Error> {
        panic!("receiving packets should not write to the transport");
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        panic!("receiving packets should not flush the transport");
    }
}

fn ready<F: Future>(future: F) -> F::Output {
    // All supplied chunks are immediately available, so Pending means the
    // client tried to read beyond them. Fail immediately instead of hanging.
    match pin!(future).poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("client waited for input despite having a complete packet"),
    }
}

fn receive_packets<const RX: usize>(chunks: &[&[u8]], discard_first_payload: bool) {
    let mut resources = ClientResources::<RX, 16>::default();
    let transport = BufferedTransport { chunks, offset: 0 };
    let mut client = Client::new(&mut resources, transport);

    let Packet::Publish(first) = ready(client.receive()).unwrap() else {
        panic!("expected first PUBLISH");
    };
    assert_eq!(first.topic, "t");
    assert_eq!(first.data_len, 2);
    if !discard_first_payload {
        let mut payload = [0; 2];
        ready(client.receive_data(&mut payload)).unwrap();
        assert_eq!(&payload, b"ON");
    }

    let Packet::Publish(second) = ready(client.receive()).unwrap() else {
        panic!("expected second PUBLISH");
    };
    assert_eq!(second.topic, "t");
    assert_eq!(second.data_len, 3);
    let mut payload = [0; 3];
    ready(client.receive_data(&mut payload)).unwrap();
    assert_eq!(&payload, b"OFF");

    assert!(matches!(
        ready(client.receive()).unwrap(),
        Packet::PingResponse
    ));
}

#[test]
fn receive_data_preserves_following_buffered_packets() {
    receive_packets::<32>(&[PACKETS], false);
}

#[test]
fn discarded_payload_does_not_delay_following_buffered_packets() {
    receive_packets::<32>(&[PACKETS], true);
}

#[test]
fn receive_and_discard_payloads_across_read_boundaries() {
    for split in 1..PACKETS.len() {
        let chunks = [&PACKETS[..split], &PACKETS[split..]];
        for discard in [false, true] {
            receive_packets::<32>(&chunks, discard);
            // Also force reads to stop at the receive buffer's capacity.
            receive_packets::<8>(&chunks, discard);
        }
    }
}
