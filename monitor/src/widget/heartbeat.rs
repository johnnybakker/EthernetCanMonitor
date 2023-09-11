use std::{time::{Instant, Duration}, collections::{self, BTreeMap}};
use common::can::{CanOpenPacket, CanPacket};
use iced::{widget::{Text, TextInput, Row, Column, text::Appearance}, Length, Color};

use crate::{widget::Widget, EventBox, can::CanPacketIn, Event};

use super::WidgetHandle;

#[derive(Debug, Clone)]
pub struct CanMessageEntry {
	id: u32,
	count: i32,
	last_received: Instant,
	delta_time: Duration
}

impl CanMessageEntry {
	pub fn new(packet: CanPacket) -> Self {
		Self {
			id: packet.id,
			count: 0, 
			last_received: Instant::now(),
			delta_time: Duration::ZERO
		}
	}

	pub fn update(&mut self, packet: &CanPacketIn) {
		self.count+=1;
		self.delta_time = packet.1 - self.last_received;
		self.last_received = packet.1;
		self.id = packet.0.id
	}
}

#[derive(Debug, Clone)]
pub struct CanMessageWidget {
	map: BTreeMap<u32, CanMessageEntry>,
}

impl Default for CanMessageWidget {
	fn default() -> Self {
        Self { map: Default::default() }
    }
}

impl CanMessageWidget {

	fn on_can_packet(&mut self, packet: &CanPacketIn) {

		let id = packet.0.id;

		let entry = match self.map.get_mut(&id) {
			Some(e) => e,
			None => {
				self.map.insert(id, CanMessageEntry::new(packet.0));
				self.map.get_mut(&id).unwrap()
			}
		};

		entry.update(packet);
	}


}

impl Widget for CanMessageWidget {

	fn setup(&self, handle: &mut WidgetHandle) {
        handle.subscribe(CanMessageWidget::on_can_packet);
    }

	fn view(&self) -> iced::Element<'static, EventBox, iced::Renderer<iced::Theme>> {

		let entries = self.map.iter().map(|entry|{
			Row::new()
			.padding(10)
			.width(Length::Fill)
			.push(
				Text::new(
					format!("{:#010x} {}, {} since last message", 
						entry.1.id, entry.1.count, entry.1.delta_time.as_micros() as f64 / 1000.0)
				)
			).into()
		})
		.collect();

		Column::with_children(entries)
		.padding(10)
		.width(Length::Fill)
		.into()
	}
}