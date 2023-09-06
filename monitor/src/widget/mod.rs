mod heartbeat;
mod handle;

pub use heartbeat::*;
pub use handle::*;

use iced::{ Element, Theme };

use crate::EventBox;

pub trait Widget {
	fn setup(&self, handle: &mut WidgetHandle);
	fn view(&self) -> Element<'static, EventBox, iced::Renderer<Theme>>;
}