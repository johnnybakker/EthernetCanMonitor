use std::{net::{UdpSocket, SocketAddr, Ipv4Addr}, time::{Duration, Instant}};
use common::can::CanPacket;

fn main() -> std::io::Result<()> {
	
	let addr = SocketAddr::from(([0, 0, 0, 0], 5001));
	let broadcast_addr = SocketAddr::new(Ipv4Addr::BROADCAST.into(), 5000);
	let socket = UdpSocket::bind(addr)?;
	socket.set_broadcast(true)?;
	socket.set_read_timeout(Some(Duration::from_millis(1000)))?;

	let heartbeat = CanPacket::new(0x720, 0, [255; 8]);
	
	let buf = unsafe {
		std::slice::from_raw_parts(
			&heartbeat as *const CanPacket as *const u8,
			std::mem::size_of::<CanPacket>()
		)
	};

	let sleep_duration = Duration::from_micros(10_000);

	// millisecond accuracy sleeper
	let spin_sleeper = spin_sleep::SpinSleeper::new(1_000_000)
    	.with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);
	
	loop {
		socket.send_to(buf, broadcast_addr)?;
		spin_sleeper.sleep(sleep_duration);
	}

}