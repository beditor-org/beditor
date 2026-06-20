pub mod codec;
pub mod connection;
pub mod multiplexer;
pub mod protocol;
// pub mod stream;
// pub mod stream_handler;
pub mod framer;
pub mod transport;
pub trait TypeName {
	fn type_name() -> &'static str {
		std::any::type_name::<Self>()
	}
}
