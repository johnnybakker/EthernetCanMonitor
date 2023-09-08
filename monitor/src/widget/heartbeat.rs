use std::time::{Instant, Duration};
use common::can::{CanOpenPacket, CanPacket};
use iced::{widget::{Text, TextInput, Row, Column}, Length};

use crate::{widget::Widget, EventBox, can::CanPacketIn, Event};

use super::WidgetHandle;

#[derive(Debug, Clone)]
enum HeartbeatWidgetEvent {
	OnNodeId(String)
}

impl Event for HeartbeatWidgetEvent {}

#[derive(Debug, Clone)]
pub struct HeartbeatWidget {
	node_id_input: String,
	node_id: u8,
	count: i32,
	last_received: Instant,
	delta_time: Duration
}

impl Default for HeartbeatWidget {
	fn default() -> Self {
        Self { 
			node_id_input: "0x30".to_owned(),
			node_id: 0x30, 
			count: 0, 
			last_received: Instant::now(),
			delta_time: Duration::ZERO
		}
    }
}

impl HeartbeatWidget {

	fn on_can_packet(&mut self, packet: &CanPacketIn) {

		if packet.0.id ^ packet.0.get_node_id() as u32 == 0x700 { 
			if self.node_id == packet.0.get_node_id() {
				self.count += 1;
				self.delta_time = packet.1 - self.last_received;
				self.last_received = packet.1;
			}
		}
	}

	fn on_event(&mut self, event: &HeartbeatWidgetEvent) {

		match event {
			HeartbeatWidgetEvent::OnNodeId(input) => self.set_node_id(input.clone()),
		}

	}

	fn set_node_id(&mut self, value: String) {
		self.node_id_input = value;

		if let Ok(node_id) = u8::from_str_radix(&self.node_id_input.trim_start_matches("0x"), 16) {
			self.node_id = node_id;
			self.count = 0;
		}

	}

}

impl Widget for HeartbeatWidget {

	fn setup(&self, handle: &mut WidgetHandle) {
        handle.subscribe(HeartbeatWidget::on_can_packet);
		handle.subscribe(HeartbeatWidget::on_event);
    }

	fn view(&self) -> iced::Element<'static, EventBox, iced::Renderer<iced::Theme>> {

		let text_input = TextInput::new("0x00", &self.node_id_input)
			.on_input(|str|HeartbeatWidgetEvent::OnNodeId(str).into())
			.width(100);

		Row::with_children(vec![
			text_input.into(),
			Text::new(
				format!("Heartbeat {:#04x}: {}, {} since last message", 
					self.node_id, self.count, self.delta_time.as_micros() as f64 / 1000.0)
			).into()
		])
		.padding(10)
		.width(Length::Fill).align_items(iced::Alignment::Center)
		.into()

	
	}
}