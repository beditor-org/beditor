use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use crate::{codec::json::JsonCodec, connection::Connection};
use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{error, info, warn};

pub trait Handler {
	fn call(&self, params: Value) -> Result<()>;
}

pub struct SyncHandler<F>(F);

impl<F> Handler for SyncHandler<F>
where
	F: Fn() -> Result<()> + 'static,
{
	fn call(&self, _: Value) -> Result<()> {
		(self.0)()
	}
}

pub struct SyncHandlerWithParams<F, P> {
	handler: F,
	_phantom: PhantomData<P>,
}

impl<F, P> SyncHandlerWithParams<F, P> {
	pub fn new(handler: F) -> Self {
		Self {
			handler,
			_phantom: PhantomData,
		}
	}
}

impl<F, P> Handler for SyncHandlerWithParams<F, P>
where
	F: Fn(P) -> Result<()> + 'static,
	P: DeserializeOwned,
{
	fn call(&self, params: Value) -> Result<()> {
		let params: P = serde_json::from_value(params)?;
		(self.handler)(params)
	}
}

/// Push-based JSON-RPC notification dispatcher.
/// The game client continuously pushes state updates to the editor;
/// the editor registers handlers per method and optionally sends
/// outgoing notifications (e.g. handshake).
pub struct JsonRpcClient {
	pub connection: Arc<Connection<JsonCodec<Value>>>,
	handlers: Option<HashMap<String, Box<dyn Handler + Send + Sync>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcNotification {
	jsonrpc: String,
	method: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	params: Option<Value>,
}

impl JsonRpcClient {
	pub fn new(connection: Connection<JsonCodec<Value>>) -> Self {
		Self {
			connection: Arc::new(connection),
			handlers: Some(HashMap::new()),
		}
	}

	pub fn listen(&mut self) {
		let connection = self.connection.clone();
		let handlers = self.handlers.take().unwrap();

		tokio::spawn(async move {
			loop {
				match connection.recv_async().await {
					Ok(message) => match serde_json::from_value::<JsonRpcNotification>(message) {
						Ok(notification) => {
							if let Some(handler) = handlers.get(&notification.method) {
								if let Err(err) = handler.call(notification.params.unwrap_or(Value::Null)) {
									error!("Error handling notification {}: {:?}", notification.method, err);
								}
							} else {
								warn!("No handler registered for method: {}", notification.method);
							}
						}
						Err(e) => {
							error!("Failed to parse JSON-RPC notification: {:?}", e);
						}
					},
					Err(error) => {
						error!("Error receiving message: {:?}", error);
						break;
					}
				}
			}
		});
		info!("JsonRpcClient listening for notifications");
	}

	pub fn notify<P: Serialize>(&self, method: &str, params: P) {
		let notification = json!({
			"jsonrpc": "2.0",
			"method": method,
			"params": params,
		});
		let _ = self.connection.send(&notification);
	}

	pub fn handle<F>(&mut self, method: &str, handler: F)
	where
		F: Fn() -> Result<()> + 'static + Send + Sync,
	{
		let f = SyncHandler(handler);
		self.handlers.as_mut().unwrap().insert(method.to_string(), Box::new(f));
	}

	pub fn handle_with_params<P, F>(&mut self, method: &str, handler: F)
	where
		P: DeserializeOwned + Send + Sync + 'static,
		F: Fn(P) -> Result<()> + 'static + Send + Sync,
	{
		let f = SyncHandlerWithParams::new(handler);
		self.handlers.as_mut().unwrap().insert(method.to_string(), Box::new(f));
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sync_handler_without_params() {
		let mut called = false;
		let handler = SyncHandler(move || Ok(()));
		assert!(handler.call(Value::Null).is_ok());
	}

	#[test]
	fn sync_handler_with_params() {
		let foo = 1;
		let handler = SyncHandlerWithParams::new(move |param: i32| {
			assert_eq!(param, foo);
			Ok(())
		});
		assert!(handler.call(json!(foo)).is_ok());
	}

	#[test]
	fn sync_handler_with_wrong_params() {
		let handler = SyncHandlerWithParams::new(move |_param: i32| Ok(()));
		assert!(handler.call(json!("not_an_int")).is_err());
	}
}
