// Bridge library for game-editor communication
// TODO: Move communication logic here

pub mod protocol;
pub mod transport;

pub use protocol::EditorProtocol;
pub use transport::EditorTransport;
