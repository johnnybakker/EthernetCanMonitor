mod event;

use std::any::TypeId;

pub use event::*;
use iced::{widget::{Row, Container}, Application};

use crate::{widget::{WidgetHandle, HeartbeatWidget, Widget}, can::{self, CanUdpSocket}};

pub struct App {
	widgets: Vec<WidgetHandle>,
	widget: Option<WidgetHandle>,
}

impl Application for App {
    
	type Executor = iced::executor::Default;
    type Message = EventBox;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		(
			Self { 
				widget: None, 
				widgets: vec![
					WidgetHandle::new(HeartbeatWidget::default())
				] 
			}, 
			iced::Command::none()
		)
    }

    fn title(&self) -> String {
        "CanMonitor".to_owned()
    }

	fn subscription(&self) -> iced::Subscription<Self::Message> {
		struct CanSocket;
		iced::subscription::channel(TypeId::of::<CanSocket>(), 0, can::udp_socket)
	}

    fn update(&mut self, e: Self::Message) -> iced::Command<Self::Message> {

		match e.unbox::<CanUdpSocket>() {
			Some(_) => {
				println!("Received sender");
			}
			None => {
				for widget in self.widgets.iter_mut() {
					widget.update(e.type_id(), e.event());
				}
			}
		}

		iced::Command::none()
	}

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
		
		let children = self.widgets.iter()
			.map(|w|w.view())
			.collect();

		let row = Row::with_children(children);
		
		Container::new(row).into()
    }
}