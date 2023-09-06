use crate::can::CanPacket;

pub trait CanOpenPacket {
	fn get_node_id(&self) -> u8;
	fn set_node_id(&mut self, node_id: u8);
}

impl CanOpenPacket for CanPacket {
    fn get_node_id(&self) -> u8 {
        (self.id & 0x7F) as u8
    }

    fn set_node_id(&mut self, node_id: u8) {
        self.id = self.id ^ 0x75 | (node_id as u32)
    }
}