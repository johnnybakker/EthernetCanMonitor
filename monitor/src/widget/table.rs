use std::{time::{Instant, Duration}, collections::BTreeMap};
use common::can::{CanOpenPacket, CanOpenType, CanPacket};
use iced::{widget::{Text, scrollable, Column, Row }, Length};

use crate::{can::CanPacketIn, widget::Widget, App, EventBox};

use super::WidgetHandle;

#[derive(Debug, Clone)]
struct CanMessageEntry {
	id: u32,
	data: [u8; 8],
	count: i32,
	last_received: Instant,
	delta_time: Duration
}

impl CanMessageEntry {
	pub fn new(packet: CanPacket) -> Self {
		Self {
			id: packet.id,
			data: packet.data,
			count: 0, 
			last_received: Instant::now(),
			delta_time: Duration::ZERO
		}
	}

	pub fn update(&mut self, packet: &CanPacketIn) {


		self.count+=1;
		self.delta_time = packet.1 - self.last_received;
		self.last_received = packet.1;
		self.id = packet.0.id;
		self.data = packet.0.data;
	}
}

#[derive(Debug, Clone)]
pub struct TableWidget {
	map: BTreeMap<u64, CanMessageEntry>,
}

impl Default for TableWidget {
	fn default() -> Self {
        Self { map: Default::default() }
    }
}

impl TableWidget {

	fn on_can_packet(&mut self, packet: &CanPacketIn) {

		let mut id = packet.0.id as u64;
		id = id << 32;
		
		match packet.0.get_type() {
			CanOpenType::SdoRequest => { 
				id = id | (packet.0.get_sdo_index() as u64) 
			}
			CanOpenType::SdoResponse => { 
				id = id | (packet.0.get_sdo_index() as u64) 
			}
			e => {}
		}

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

impl Widget for TableWidget {

	fn setup(&self, handle: &mut WidgetHandle) {
        handle.subscribe(TableWidget::on_can_packet);
    }

	fn view(&self, handle: &WidgetHandle) -> iced::Element<'static, EventBox, iced::Renderer<iced::Theme>> {

		let mut entries = Vec::new();
		entries.push(
			Row::new()
			.padding(10)
			.width(Length::Fill)
			.push(
				Column::new().push(
					Text::new("ID").width(100)
				)
			)
			.push(
				Column::new().push(
					Text::new("COUNT")
				).width(100)
			).push(
				Column::new().push(
					Text::new("INTERVAL (MS)")
				).width(200)
			).push(
				Column::new().push(
					Text::new("DATA")
				)
			)
			.into()
		);

		let mut content = Column::with_children(entries)
			.padding(10)
			.width(Length::Fill);

		for entry in self.map.iter() {
			
			let data: Vec<String> = entry.1.data.iter().map(|d|format!("{:#02x}", d)).collect();
			let data_string = data.join(", ");

			content = content.push(
				Row::new()
				.padding(10)
				.width(Length::Fill)
				.push(
					Column::new().push(
						Text::new(
							format!("{:#010x}", entry.1.id)
						).width(100)
					)
				)
				.push(
					Column::new().push(
						Text::new(entry.1.count.to_string())
					).width(100)
				).push(
					Column::new().push(
						Text::new((entry.1.delta_time.as_micros() as f64 / 1000.0).to_string())
					).width(200)
				).push(
					Column::new().push(
						Text::new(data_string)
					)
				)
			);

		}
				
		scrollable(content).width(Length::Fill).into()
	}
}