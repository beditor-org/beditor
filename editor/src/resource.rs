use std::{
	any::{Any, TypeId},
	collections::HashMap,
	sync::{Arc, RwLock},
};

use crate::event::Events;

#[derive(Clone)]
pub struct ResourceRegistry {
	resources: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}

impl ResourceRegistry {
	pub fn new() -> Self {
		let registry = Self {
			resources: Arc::new(RwLock::new(HashMap::new())),
		};
		registry.register(Events::new());
		registry
	}

	pub fn register<T: 'static + Send + Sync>(&self, resource: T) {
		self.resources
			.write()
			.unwrap()
			.insert(TypeId::of::<T>(), Box::new(Arc::new(resource)));
	}

	pub fn get<T: 'static + Send + Sync>(&self) -> Option<Arc<T>> {
		self.resources
			.read()
			.unwrap()
			.get(&TypeId::of::<T>())
			.and_then(|boxed| boxed.downcast_ref::<Arc<T>>())
			.map(Arc::clone)
	}

	pub fn unregister<T: 'static + Send + Sync>(&self) -> Option<Arc<T>> {
		self.resources
			.write()
			.unwrap()
			.remove(&TypeId::of::<T>())
			.and_then(|boxed| boxed.downcast::<Arc<T>>().ok())
			.map(|arc| *arc)
	}
}
