use std::{collections::HashMap, io::Write, sync::Arc};

use crate::{codec::json::JsonCodec, connection::Connection};
use anyhow::Result;
use flume::{unbounded, Sender};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum JsonRpcMessage {
	Request(JsonRpcRequest),
	Response(JsonRpcResponse),
}

pub struct JsonRpcClient<W: Write> {
	connection: Arc<Connection<JsonCodec, W>>,
	pending: Arc<Mutex<HashMap<u64, Sender<Value>>>>,
	handlers: HashMap<String, Box<dyn FnMut(Value)>>,
	next_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcRequest {
	jsonrpc: String,
	method: String,
	params: Value,
	id: u64,
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
			handlers: HashMap::new(),
			next_id: 0,
		}
	}

	pub fn run(&mut self) {
		let pending = self.pending.clone();
		let connection = self.connection.clone();

		tokio::spawn(async move {
			loop {
				match connection.recv_async().await {
					Ok(message) => {
						match serde_json::from_value::<JsonRpcMessage>(message) {
							Ok(JsonRpcMessage::Request(request)) => {
								debug!("Received request: {:?}", request);
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

	pub async fn call<P: Serialize, R: DeserializeOwned + Serialize>(&mut self, method: &str, params: P) -> Result<R> {
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

	pub fn add_handler<F>(&mut self, method: &str, handler: F)
	where
		F: FnMut(Value) + 'static,
	{
		self.handlers.insert(method.to_string(), Box::new(handler));
	}
}
