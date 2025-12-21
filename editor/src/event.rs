use std::{
	any::{Any, TypeId},
	collections::HashMap,
	sync::{Arc, RwLock},
};

pub struct DumyEvent;
pub struct OpenGameEvent(pub String);

#[derive(Clone)]
pub struct Events {
	handlers: Arc<RwLock<HashMap<TypeId, Vec<Box<dyn Fn(&dyn Any) + Send + Sync>>>>>,
}

impl Events {
	pub fn new() -> Self {
		Self {
			handlers: Arc::new(RwLock::new(HashMap::new())),
		}
	}

	pub fn subscribe<E: 'static>(&self, handler: impl Fn(&E) + Send + Sync + 'static) {
		let type_id = TypeId::of::<E>();

		let boxed_handler = Box::new(move |event: &dyn Any| {
			if let Some(concrete_event) = event.downcast_ref::<E>() {
				handler(concrete_event);
			}
		});

		let mut handlers = self.handlers.write().unwrap();
		handlers.entry(type_id).or_insert_with(Vec::new).push(boxed_handler);
	}

	pub fn publish<E: 'static>(&self, event: E) {
		let type_id = TypeId::of::<E>();

		let handlers = self.handlers.read().unwrap();
		if let Some(event_handlers) = handlers.get(&type_id) {
			for handler in event_handlers {
				handler(&event as &dyn Any);
			}
		}
	}
}
