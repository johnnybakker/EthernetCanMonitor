use std::{net::{UdpSocket, SocketAddr, Ipv4Addr}, time::{Duration, Instant}};
use common::can::{CanOpenPacket, CanPacket};
use rand::{distributions::Uniform, prelude::Distribution};

fn main() -> std::io::Result<()> {
	
	let addr = SocketAddr::from(([0, 0, 0, 0], 5001));
	let broadcast_addr = SocketAddr::from(([192, 168, 144, 255], 5001));
	let socket = UdpSocket::bind(addr)?;
	socket.set_broadcast(true)?;
	socket.set_read_timeout(Some(Duration::from_millis(1000)))?;


	let mut heartbeat = CanPacket::new(0x640, 0, [0; 8]);
	heartbeat.set_cmd(0x20);
	heartbeat.set_index(0x1004);
	heartbeat.set_rtr(false);
	heartbeat.set_extended(false);
	heartbeat.set_sdo_length(1);
	heartbeat.set_subindex(0);
	heartbeat.data[4] = 2;
	
	let buf = unsafe {
		std::slice::from_raw_parts(
			&heartbeat as *const CanPacket as *const u8,
			std::mem::size_of::<CanPacket>()
		)
	};

	println!("{:?}", buf);

	socket.send_to(buf, broadcast_addr)?;


	loop {}

	// // millisecond accuracy sleeper
	// let spin_sleeper = spin_sleep::SpinSleeper::new(1_000_000)
    // 	.with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);

	// let range = Uniform::new(10u64, 500);
	// let mut rng = rand::thread_rng();

	// loop {
	// 	let sleep_duration = Duration::from_millis(range.sample(&mut rng));

	// 	socket.send_to(buf, broadcast_addr)?;
	// 	spin_sleeper.sleep(sleep_duration);
	// }

}