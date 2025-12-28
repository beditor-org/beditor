use std::collections::HashMap;

use crate::{
	panel::{SocketAlignment, SocketsConfig},
	PanelSocket,
};

#[derive(Clone, Debug, Default)]
pub struct WindowConfig {
	pub title: String,
	pub decorations: bool,
	pub resizable: bool,
}

#[derive(Clone, Debug)]
pub struct EditorConfig {
	pub sockets: SocketsConfig,
	pub window: WindowConfig,
}

impl Default for EditorConfig {
	fn default() -> Self {
		Self {
			window: WindowConfig {
				title: format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
				resizable: true,
				..Default::default()
			},
			sockets: HashMap::from_iter([
				(PanelSocket::Left, SocketAlignment::LeftToRight),
				(PanelSocket::Right, SocketAlignment::RightToLeft),
				(PanelSocket::Top, SocketAlignment::TopToBottom),
				(PanelSocket::Bottom, SocketAlignment::BottomToTop),
				(PanelSocket::Center, SocketAlignment::LeftToRight),
				(PanelSocket::CenterTop, SocketAlignment::TopToBottom),
				(PanelSocket::CenterBottom, SocketAlignment::TopToBottom),
			]),
		}
	}
}
