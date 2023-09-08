use std::{cell::RefCell, rc::Rc, any::TypeId};
use iced::{Element, Theme, widget::Row};
use crate::{Event, EventBox};
use super::Widget;


pub struct WidgetHandle {  
	pub id: usize,
	pub is_pinned: bool,
	widget: Rc<RefCell<dyn Widget>>,
	subscriptions: Vec<(TypeId, Box<dyn Fn(&mut dyn Widget, &dyn Event)>)>
}

impl WidgetHandle {

	pub fn new(id: usize, widget: impl Widget + 'static) -> Self {
	
		let widget = Rc::new(RefCell::new(widget));

		let mut handle = Self {
			id: id,
			is_pinned: false,
			widget: widget.clone(),
			subscriptions: Vec::default()
		};

		widget.borrow_mut().setup(&mut handle);
		handle
	}

	fn get_mut_widget(&self) -> &mut dyn Widget {
		unsafe { &mut *self.widget.as_ptr() }
	}

	pub fn update(&mut self, type_id: TypeId, e: &dyn Event) -> Option<iced::Command<EventBox>> {
		
		for subscription in self.subscriptions.iter() {
			if subscription.0 == type_id {
				subscription.1(self.get_mut_widget(), e);
			}
		}

		None
	}

	pub fn view(&self) -> Element<'static, EventBox, iced::Renderer<Theme>> {
		Row::new().push(self.get_mut_widget().view()).into()
	}
	
	pub fn subscribe<W: Widget + 'static, T: Event + 'static>(&mut self, handler: fn(&mut W, &T)) {

		let type_id = TypeId::of::<T>();
		
		let handler = move |w: &mut dyn Widget, e: &dyn Event| unsafe { 
			handler(
				&mut *(w as *mut dyn Widget as *mut W),
				&*(e as *const dyn Event as *const T)
			) 
		};

		self.subscriptions.push((type_id, Box::new(handler)));
	}

}
