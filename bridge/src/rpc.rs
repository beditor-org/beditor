use std::{collections::HashMap, io::Write, marker::PhantomData, sync::Arc};

use crate::{codec::json::JsonCodec, connection::Connection};
use anyhow::Result;
use flume::{unbounded, Sender};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, error, info, warn};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum JsonRpcMessage {
	Request(JsonRpcRequest),
	Response(JsonRpcResponse),
}

pub trait Handler {
	fn call(&self, params: Value) -> Result<()>;
}

pub struct SyncHandler<F>(F)
where
	F: Fn() + 'static;

impl<F> Handler for SyncHandler<F>
where
	F: Fn() + 'static,
{
	fn call(&self, _: Value) -> Result<()> {
		(self.0)();
		Ok(())
	}
}

pub struct SyncHandlerWithParams<P, F>(F, PhantomData<P>)
where
	P: DeserializeOwned,
	F: Fn(P) + 'static;

impl<P: DeserializeOwned, F> Handler for SyncHandlerWithParams<P, F>
where
	F: Fn(P) + 'static,
{
	fn call(&self, params: Value) -> Result<()> {
		let params: P = serde_json::from_value(params)?;
		(self.0)(params);
		Ok(())
	}
}

pub struct JsonRpcClient<W: Write> {
	connection: Arc<Connection<JsonCodec, W>>,
	pending: Arc<Mutex<HashMap<u64, Sender<Value>>>>,
	handlers: Option<HashMap<String, Box<dyn Handler + Send + Sync>>>,
	next_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcRequest {
	jsonrpc: String,
	method: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	params: Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcResponse {
	jsonrpc: String,
	result: Value,
	id: u64,
}

impl<W: Write + Send + 'static> JsonRpcClient<W> {
	pub fn new(connection: Connection<JsonCodec, W>) -> Self {
		Self {
			connection: Arc::new(connection),
			pending: Arc::new(Mutex::new(HashMap::new())),
			handlers: Some(HashMap::new()),
			next_id: 0,
		}
	}

	pub fn run(&mut self) {
		let pending = self.pending.clone();
		let connection = self.connection.clone();
		let handlers = Arc::new(RwLock::const_new(self.handlers.take().unwrap()));

		tokio::spawn(async move {
			loop {
				match connection.recv_async().await {
					Ok(message) => {
						match serde_json::from_value::<JsonRpcMessage>(message) {
							Ok(JsonRpcMessage::Request(request)) => {
								info!("Received request: {} with params {:?}", request.method, request.params);
								if let Some(handler) = handlers.read().await.get(&request.method) {
									if let Err(err) = handler.call(request.params.unwrap_or(Value::Null)) {
										error!("Error handling request {}: {:?}", request.method, err);
									}
								} else {
									warn!("No handler registered for method: {}", request.method);
								}
							}
							Ok(JsonRpcMessage::Response(response)) => {
								debug!("Received response: {:?}", response);
								match pending.lock().await.remove(&response.id) {
									Some(tx) => {
										tx.send(response.result).unwrap_or_else(|e| {
											error!("Failed to send response for request {}: {:?}", response.id, e)
										});
									}
									None => {
										warn!("No pending request found for id {}", response.id);
									}
								}
							}
							Err(e) => {
								error!("Failed to parse JSON RPC message: {:?}", e);
							}
						};
					}
					Err(error) => {
						error!("Error receiving message: {:?}", error);
						break; //	is it needed or we can continue?
					}
				}
			}
		});
		info!("JSONRpcClient initialized and listening for responses");
	}

	pub async fn request<P: Serialize, R: DeserializeOwned + Serialize>(&mut self, method: &str, params: P) -> Result<R> {
		self.next_id += 1;

		let request = json!({
			"jsonrpc": "2.0",
			"method": method,
			"params": params,
			"id": self.next_id,
		});

		self.connection.send(request);
		let (tx, rx) = unbounded();
		self.pending.lock().await.insert(self.next_id, tx).unwrap();
		let raw_response = rx.recv_async().await?;
		let response = serde_json::from_value::<JsonRpcResponse>(raw_response)?.result;
		serde_json::from_value::<R>(response).map_err(|e| e.into())
	}

	pub fn notify<P: Serialize>(&self, method: &str, params: P) {
		let notification = json!({
			"jsonrpc": "2.0",
			"method": method,
			"params": params,
		});

		self.connection.send(notification);
	}

	pub fn handle<F>(&mut self, method: &str, handler: F)
	where
		F: Fn() + 'static + Send + Sync,
	{
		let f = SyncHandler(handler);
		let boxed = Box::new(f);
		self.handlers.as_mut().unwrap().insert(method.to_string(), boxed);
	}

	pub fn handle_with_params<P: 'static, F>(&mut self, method: &str, handler: F)
	where
		P: DeserializeOwned + Sync + Send + 'static,
		F: Fn(P) + 'static + Send + Sync,
	{
		let f = SyncHandlerWithParams(handler, PhantomData);
		let boxed = Box::new(f);
		self.handlers.as_mut().unwrap().insert(method.to_string(), boxed);
	}
}
