use std::{net::{UdpSocket, SocketAddr, Ipv4Addr}, time::{Duration, Instant}};
use common::can::{CanPacket, XLCanPacket, XLCanPacketHeader, XLCanPacketType};
use rand::{distributions::Uniform, prelude::Distribution};

fn main() -> std::io::Result<()> {
	
	let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
	let broadcast_addr = SocketAddr::new([172,19,220,122].into(), 5000);
	let socket = UdpSocket::bind(addr)?;
	socket.set_broadcast(true)?;
	socket.set_read_timeout(Some(Duration::from_millis(1000)))?;

	const DATA_SIZE: usize = 10;

	let xl = XLCanPacket {
		header: XLCanPacketHeader { 
			id: 0, 
			packet_type: XLCanPacketType::UnknownMessage, 
			node_id: 0x30, 
			length: DATA_SIZE as u16
		},
		data: Box::new([5; DATA_SIZE])
	};

	const HEADER_SIZE: usize = std::mem::size_of::<XLCanPacketHeader>();
	
	const TOTAL_SIZE: usize = HEADER_SIZE + DATA_SIZE;



	let buf = unsafe {

		let mut buf = [0; TOTAL_SIZE];

		std::ptr::copy(
			&xl.header as *const XLCanPacketHeader, 
			buf.as_mut_ptr() as *mut XLCanPacketHeader, 
			1
		);

		std::ptr::copy(
			(*xl.data).as_ptr(), 
			buf[HEADER_SIZE..].as_ptr() as *mut u8, 
			DATA_SIZE
		);

		buf
	};

	// millisecond accuracy sleeper
	let spin_sleeper = spin_sleep::SpinSleeper::new(1_000_000)
    	.with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);

	let range = Uniform::new(10u64, 500);
	let mut rng = rand::thread_rng();


	let mut buf = [0u8; u16::MAX as usize];

	loop {
	
		match socket.recv_from(&mut buf) {
			Ok((size, _addr)) => unsafe {

				if size != std::mem::size_of::<CanPacket>() {
					
					let header = *(
						buf[0..std::mem::size_of::<XLCanPacketHeader>()].as_ptr() as *const XLCanPacketHeader
					);

					let id = header.id;
					let node_id = header.node_id;
					let length = header.length;

					let data = &buf[std::mem::size_of::<XLCanPacketHeader>()..];
					//if node_id != data[0] {
						println!("Received {}, {}", node_id, data[0]);
					//}
				}

				//Some()
			},
			Err(_) => {
				
			
			}
		};


	}

}