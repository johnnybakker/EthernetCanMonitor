#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct CanPacket {
	pub id: u32,
	pub flags: u8,
	pub data: [u8; 8],
	pub crc: u8
}

impl CanPacket {
	pub fn new(id: u32, flags: u8, data: [u8; 8]) -> Self {
		Self { id, flags, data, crc: 0 }
	}
}