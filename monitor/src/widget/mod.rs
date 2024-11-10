mod graph;
mod table;
mod handle;
mod sdo_request_button;

pub use graph::*;
pub use table::*;
pub use handle::*;
pub use sdo_request_button::*;

use iced::{ Element, Theme };

use crate::{App, EventBox};

pub trait Widget {
	fn setup(&self, handle: &mut WidgetHandle);
	fn view(&self, handle: &WidgetHandle) -> Element<'static, EventBox, iced::Renderer<Theme>>;
}