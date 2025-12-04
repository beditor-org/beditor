## Usage
- do not directly attach  DefaultPlugins to your app. use `app.with_default_plugins` instead:

```rust
app.with_default_plugins(DefaultPlugins.set(WindowPlugin {
	primary_window: Some(Window {
		title: "🎮 Game Viewport".to_string(),
		..default()
	}),
	..default()
}))
```
- call app.with_editor_plugins() to conditionally attach editor-specific stuff
- add EditorMainCamera for main camera in your game
