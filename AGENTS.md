# Agent Rules for beditor Project

## Code Style & Formatting

- respect formatting rules from `rustfmt.toml`
<!-- - **Max line length: 130 characters** -->
- Run `cargo fmt` before suggesting code
<!-- - Use `eprintln!` for debug/logging, never `println!` -->
- Add final newline to all files
- use english to write comments
- stop using fucking emogis
<!-- ## Rust Best Practices

- Prefer pattern matching over if-let chains when multiple cases exist
- Use `try_recv()` instead of blocking `recv()` in performance-critical paths
- Avoid unnecessary cloning - prefer borrowing
- Use meaningful variable names (no single-letter vars except in closures/loops)
- Add doc comments (`///`) for public APIs

## Architecture Decisions

### BRP Protocol (Bevy Remote Protocol)
- Always use fully qualified component names: `"bevy_ecs::name::Name"`, not `"Name"`
- Method name is `"world.query"`, not `"bevy/query"`
- Document expected response format in comments
- Handle both Result and Error payloads

### IPC Communication
- Game and editor communicate via stdin/stdout
- JSON-RPC 2.0 format for all messages
- Non-blocking reads only (`try_recv`, not `recv`)
- Editor polls state every 500ms for UI updates

### Performance
- Game uses `FixedUpdate` at 10Hz for BRP request processing
- VSync enabled (`PresentMode::AutoVsync`)
- Early return from systems when no work needed
- Use `WinitSettings::reactive()` to reduce idle CPU

## UI/UX

- Left panel: Entity hierarchy
- Right panel: Inspector (properties)
- Update UI reactively using Dioxus signals
- Show connection status in UI

## Error Handling

- Log BRP errors to stderr with context
- Don't panic on communication errors
- Gracefully handle game disconnect

## Testing Workflow

- Always test both editor and game together
- Check CPU usage in `htop` (press `H` to hide threads)
- Verify entity list updates in UI
- Test with game window focused and unfocused

## Commands to Run

```bash
# Build both projects
cargo build --manifest-path /projects/bevy_demo_game/Cargo.toml
cargo build --manifest-path /projects/beditor/Cargo.toml

# Run editor (automatically launches game)
cd /projects/beditor && cargo run

# Format code
cargo fmt --all
```

## Known Issues & Limitations

- Entity updates require polling (no push notifications yet)
- Name component must exist for entities to appear in list
- Multiple threads shown in htop is normal (Bevy's task system) -->
