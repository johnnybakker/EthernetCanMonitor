mod graph;
mod table;
mod handle;

pub use graph::*;
pub use table::*;
pub use handle::*;

use iced::{ Element, Theme };

use crate::EventBox;

pub trait Widget {
	fn setup(&self, handle: &mut WidgetHandle);
	fn view(&self) -> Element<'static, EventBox, iced::Renderer<Theme>>;
}