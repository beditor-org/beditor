## IPC
for IPC following terms are used:
all data incoming and outgoing from app is just a stream - everything that implements AsyncRead/AsyncWrite
stream + framer = transport

transport gives flume channels to read/write

next you can use multiplexer and split data further for channels

or pass it to connection

connection is then used in protocols

Protocol is used as a wraper around connection.send. If you don't send anything, you don't need protocol, use Connection instead.

codec + transport flume channels = connection

each protocol shares some boilerplate:
```rust
pub struct Protocol<C: Codec> {
	connection: Connection<C>,
}

impl<C: Codec> Protocol<C> {
	pub fn send(&self, msg: C::Message) {
		self.connection.send(msg);
	}

	pub fn try_recv(&self) -> Option<C::Message> {
		self.connection.try_recv().ok().flatten()
	}
}
```

Inside app some plugin should create and start using protocol, fully owning it. Then it should register some resources using `use_context_provider`. This protocol plugin is responsible for creating and updating such resources as well as reacting on resource updates and sending data via protocol.
