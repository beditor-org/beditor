use std::{
	any::{Any, TypeId},
	cell::RefCell,
	collections::HashMap,
	rc::Rc,
};

pub struct DumyEvent;
pub struct OpenGameEvent(pub String);

#[derive(Clone)]
pub struct Events {
	handlers: Rc<RefCell<HashMap<TypeId, Vec<Box<dyn FnMut(&dyn Any)>>>>>,
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}

impl Events {
	pub fn new() -> Self {
		Self {
			handlers: Rc::new(RefCell::new(HashMap::new())),
		}
	}

	pub fn subscribe<E: 'static>(&self, mut handler: impl FnMut(&E) + 'static) {
		let type_id = TypeId::of::<E>();

		let boxed_handler = Box::new(move |event: &dyn Any| {
			if let Some(concrete_event) = event.downcast_ref::<E>() {
				handler(concrete_event);
			}
		});

		let mut handlers = self.handlers.borrow_mut();
		handlers.entry(type_id).or_default().push(boxed_handler);
	}

	pub fn publish<E: 'static>(&self, event: E) {
		let type_id = TypeId::of::<E>();

		let mut handlers = self.handlers.borrow_mut();
		if let Some(event_handlers) = handlers.get_mut(&type_id) {
			for handler in event_handlers {
				handler(&event as &dyn Any);
			}
		}
	}
}
