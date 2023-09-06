use std::{net::{SocketAddr, UdpSocket, Ipv4Addr}, convert::Infallible, time::{Duration, Instant}};

use common::can::{CanPacket, CanOpenPacket};
use iced::futures::{channel::mpsc::{self, Sender, channel}, SinkExt};

use crate::{EventBox, Event};

use super::CanPacketIn;

#[derive(Debug, Clone)]
pub struct CanUdpSocket(pub Sender<CanPacket>);
impl Event for CanUdpSocket {}

pub async fn udp_socket(mut output: mpsc::Sender<EventBox>) -> Infallible {

	let mut buf = [0u8; std::mem::size_of::<CanPacket>()];
	let addr = SocketAddr::from(([0, 0, 0, 0], 5000));

	let (tx, mut rx) = channel::<CanPacket>(0);
	let _ = output.send(CanUdpSocket(tx).into()).await;

	let socket = UdpSocket::bind(addr).unwrap();
	socket.set_broadcast(true).unwrap();
	socket.set_read_timeout(Some(Duration::from_micros(10))).unwrap();

	let broadcast_addr = SocketAddr::new(Ipv4Addr::BROADCAST.into(), 5000);

	loop {

		if let Some(msg) = rx.try_next().unwrap_or_default() {
			let buf = unsafe {
				std::slice::from_raw_parts(
					&msg as *const CanPacket as *const u8,
					std::mem::size_of::<CanPacket>()
				)
			};

			let _ = socket.send_to(buf, broadcast_addr);
		}
	

		let msg = match socket.recv_from(&mut buf) {
			Ok((_size, _addr)) => unsafe {
				Some(*(buf.as_slice() as *const [u8] as *const CanPacket))
			},
			Err(_) => None,
		};

		
	
		if let Some(msg) = msg {
			let event = EventBox::from(CanPacketIn(msg, Instant::now()));
			let _ = output.send(event).await;
		}
	}
}