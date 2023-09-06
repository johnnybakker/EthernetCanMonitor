use core::fmt::Debug;
use std::{any::TypeId, sync::Arc};

pub trait Event: Debug + Sync + Send { }

impl<T: Event + 'static> From<T> for EventBox {
    fn from(value: T) -> Self {
        EventBox::new(value)
    }
}

#[derive(Debug, Clone)]
pub struct EventBox {
	type_id: TypeId,
	event: Arc<dyn Event>
}

impl EventBox {
	pub fn new<T: Event + 'static>(event: T) -> Self {
		Self {
			type_id: TypeId::of::<T>(),
			event: Arc::new(event)
		}
	}

	pub fn is<T: Event + 'static>(&self) -> bool {
		TypeId::of::<T>() == self.type_id
	}

	#[inline]
	pub fn unbox<T: Event + 'static>(&self) -> Option<&T> {
		if self.is::<T>() {
			Some(unsafe {
				let event = &*self.event;
				&*(event as *const dyn Event as *const T)
			})
		} else {
			None
		}
	}

	#[inline]
	pub fn unbox_unchecked<T: Event>(&self) -> &T {
		unsafe {
			let event = &*self.event;
			&*(event as *const dyn Event as *const T)
		}
	}

	pub fn type_id(&self) -> TypeId {
		self.type_id
	}

	pub fn event(&self) -> &dyn Event {
		&*self.event
	}
}

