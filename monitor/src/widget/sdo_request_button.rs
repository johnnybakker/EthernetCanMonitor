use std::sync::Arc;

use common::can::{ CanPacket, CanOpenPacket };
use iced::widget::{button, canvas::path::lyon_path::geom::euclid::default, row, text, Button};

use crate::{App, AppMessage, EventBox};

use super::{Widget, WidgetHandle};


#[derive(Default, Debug)]
pub struct SDORequestButton {

}	



impl Widget for SDORequestButton {
	fn setup(&self, handle: &mut WidgetHandle) {
	
	}

	fn view(&self, handle: &WidgetHandle) -> iced::Element<'static, crate::EventBox, iced::Renderer<iced::Theme>> {
		if handle.is_pinned {
			
			let mut packet = CanPacket::new(0x580, 0, [0;8]);
			packet.set_node_id(0x40);
			packet.set_cmd(0x10);
			packet.set_index(100);
			packet.set_subindex(0);
			packet.set_data_length(1);
			packet.data[0] = 0x01;
			

			let send_packet = AppMessage::SendPacket(packet);
			
			Button::new("Send").on_press(send_packet.into()).into()

		} else {
			todo!()
		}
	}
}