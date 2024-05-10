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

	pub fn set_rtr(&mut self, value: bool) {
		self.flags = self.flags & 0b11111110;
		if value {
			self.flags |= 0b00000001;
		}
	}

	pub fn set_extended(&mut self, value: bool) {
		self.flags = self.flags & 0b11111101;
		if value {
			self.flags |= 0b00000010;
		}
	}

	pub fn set_data_length(&mut self, length: u8) {
		self.flags = self.flags & 0b00001111;
		self.flags |= (length << 4) & 0b11110000;
	}
}

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum XLCanPacketType {
	StatusMessage = 0,
	UnknownMessage = 1,
}

#[repr(C, packed)]
pub struct XLCanPacket {
	pub header: XLCanPacketHeader,
	pub data: Box<[u8]>
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct XLCanPacketHeader {
	pub id : u16,
	pub packet_type: XLCanPacketType,
	pub node_id: u8,
	pub length: u16,
}


impl Default for XLCanPacketHeader {
    fn default() -> Self {
        Self { 
			id: 0, 
			packet_type: XLCanPacketType::UnknownMessage, 
			node_id: 0, 
			length: 0
		}
    }
}
