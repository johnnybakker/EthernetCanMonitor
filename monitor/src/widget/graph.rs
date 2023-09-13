use std::{f32::consts::PI, collections::BTreeMap, time::{Instant, Duration}};

use iced::{widget::{Column, canvas::{Geometry, self, Frame, Stroke, Fill}}, Length, Renderer, Point, Color, Vector};

use crate::{widget::Widget, EventBox, can::CanPacketIn};
use common::can::CanOpenPacket;

use super::WidgetHandle;

struct MyProgram {
	data: BTreeMap<u64, u64>
}

struct ProgramState;


impl Default for ProgramState {
    fn default() -> Self {
        Self {  }
    }
}

impl iced::widget::canvas::Program<EventBox, Renderer<iced::Theme>> for MyProgram {
    type State = ProgramState;



    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer<iced::Theme>,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<<Renderer<iced::Theme> as iced::widget::canvas::Renderer>::Geometry> {

		let mut frame = Frame::new(renderer, bounds.size());
	
		

		let res_x = 40.0;
		let res_y = 20.0;

		let stroke = Stroke::default().with_color(Color::BLACK).with_width(1.0);
		
		let offset_x = 25.0;
		let offset_y = 15.0;
		
		let max_x = ((frame.width()-offset_x) / res_x) as i32;
		let max_y = ((frame.height()-offset_y) / res_y) as i32;

		let w = max_x as f32 *res_x + offset_x;
		let h = max_y as f32 *res_y + offset_y;

		let label_offset_x = -6.0;
		let label_offset_y = 11.0;
		
		let keys: Vec<&u64> = self.data.keys().collect();
		
		let from = if keys.len() > max_x as usize {
			keys.len() - max_x as usize
		} else {
			0usize
		};

		let to = if keys.len() > max_x as usize {
			from + max_x as usize
		} else {
			keys.len()
		};

		for i in 0 .. max_x + 1 {
			let x = i as f32 * res_x + offset_x;
			frame.stroke(&canvas::Path::line([x, frame.height()-offset_y].into(), [x, frame.height()-h].into()), stroke.clone());
			
			let label_x = i as f32 * res_x + offset_x + label_offset_x;
			frame.with_save(|frame| {
				frame.translate([label_x, frame.height()-offset_y].into());
				frame.fill_text(format!("{}", (i as usize+from)));
			});
		}

		for j in 0 .. max_y + 1 {
			let y = j as f32 * res_y + offset_y;
			frame.stroke(&canvas::Path::line([offset_x, frame.height()-y-1.0].into(), [w, frame.height()-y-1.0].into()), stroke.clone());
		
		
			let label_y = j as f32 * res_y + offset_y + label_offset_y;
			frame.with_save(|frame| {
				frame.translate([0.0, frame.height()-label_y].into());
				frame.fill_text(format!("{}", j));
			});
			
		}

		let data = canvas::Path::new(|b|{
			for i in from..to {
				let key = keys[i];
				let x = (i - from) as f32 * res_x + offset_x;
				let y = res_y * self.data[key] as f32 + offset_y;

				b.line_to([x, frame.height() - y].into());
			}
		});

		frame.stroke(&data, Stroke::default().with_color(Color::from_rgb8(0, 0, 255)).with_width(2.5));


		//frame.rotate(PI*0.5);

		

        vec![
			frame.into_geometry()
		]
    }

 


}

#[derive(Debug, Clone)]
struct CanMessageEntry {
	count: u64
}

pub struct GraphWidget {
	data: BTreeMap<u64, u64>,
	start: Instant
}

impl GraphWidget {

	fn on_can_packet(&mut self, _packet: &CanPacketIn) {

		if _packet.0.id ^ _packet.0.get_node_id() as u32 != 0x100 {
			return;
		}

		let last_packet_time = _packet.1;
		let duration = last_packet_time.duration_since(self.start);
		let key = duration.as_secs() as u64;

		let v = if let Some(v) = self.data.get_mut(&key) {
			v
		} else {
			self.data.insert(key, 0);
			self.data.get_mut(&key).unwrap()
		};

		(*v) += 1;
	}

}

impl Widget for GraphWidget {

	fn setup(&self, handle: &mut WidgetHandle) {
        handle.subscribe(GraphWidget::on_can_packet);
    }

	fn view(&self) -> iced::Element<'static, EventBox, iced::Renderer<iced::Theme>> {

		let canvas = iced::widget::Canvas::new(
			MyProgram {
				data: self.data.clone()
			})
			.width(Length::Fill)
			.height(Length::Fill);
	
		canvas.into()
	}
}

impl Default for GraphWidget {
    fn default() -> Self {
        Self { data: Default::default(), start: Instant::now() }
    }
}