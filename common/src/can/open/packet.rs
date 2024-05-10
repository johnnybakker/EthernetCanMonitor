use crate::can::CanPacket;


pub enum CanOpenType {
	Heartbeat = 0,
	SdoResponse = 1,
	SdoRequest = 2,
	TransmitPdo = 3,
	ReceivePdo = 4,
	Unknown = 5
}

pub trait CanOpenPacket {
	fn get_node_id(&self) -> u8;
	fn set_node_id(&mut self, node_id: u8);
	fn set_cmd(&mut self, cmd: u8);
	fn set_index(&mut self, index: u16);
	fn set_subindex(&mut self, subindex: u8);
	fn set_sdo_length(&mut self, cmd: u8);
	fn get_sdo_index(&self) -> u16;
	fn get_sdo_subindex(&self) -> u8;
	fn get_type(&self)  -> CanOpenType;
}

impl CanOpenPacket for CanPacket {
    fn get_node_id(&self) -> u8 {
        (self.id & 0x7F) as u8
    }

    fn set_node_id(&mut self, node_id: u8) {
        self.id = self.id ^ 0x75 | (node_id as u32)
    }
	
	fn set_index(&mut self, index: u16) {
		self.data[1] = (index >> 0) as u8;
		self.data[2] = (index >> 8) as u8;
	}
	
	fn set_subindex(&mut self, subindex: u8) {
		self.data[3] = subindex;
	}
	
	fn set_cmd(&mut self, cmd: u8) {
		self.data[0] = (self.data[0] & 0b00011111) | cmd & 0b11100000;
	}

	fn set_sdo_length(&mut self, length: u8) {
		self.set_data_length(8);
		self.data[0] = (self.data[0] & 0xF3) | (((4 - length) & 0x0F) << 2) | 0b11;
	}
	
	fn get_type(&self)  -> CanOpenType {
		let id = self.id & !0x7F;
		if id == 0x700 {
			CanOpenType::Heartbeat 
		} else if id >= 0x580 && id < 0x600 {
			CanOpenType::SdoRequest
		} 
		else if id >= 0x600 && id < 0x6E0 {
			CanOpenType::SdoRequest
		} 
		else {
			CanOpenType::Unknown
		}
	}
	
	fn get_sdo_index(&self) -> u16 {
		(self.data[1] as u16) << 8 | (self.data[2] as u16)
	}
	
	fn get_sdo_subindex(&self) -> u8 {
		self.data[3]
	}
}