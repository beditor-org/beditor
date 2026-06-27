# beditor - Bevy Game Editor

Extensible game editor for Bevy engine built with Dioxus.

## Features

- 🔌 **Plugin Architecture** - Extensible through plugins
- 🎮 **Viewport Rendering** - Multiple rendering methods to choose from
- 🎨 **Theming** - Dark/Light themes support
- 📦 **Modular** - Clean separation of concerns

## Viewport Rendering System

beditor uses a plugin-based approach for viewport rendering, allowing you to choose the best method for your platform and workflow:

- **Custom Protocol** (Recommended) - Cross-platform, reliable, ~60 FPS
- **Shared Memory** (High Performance) - Linux/Windows optimized, ~120 FPS
- **Window Overlay** (Legacy) - Native rendering but has tiling WM issues

See [VIEWPORT_PLUGIN_QUICKSTART.md](./VIEWPORT_PLUGIN_QUICKSTART.md) for details.

## Viewport Controls

Controls are active when the mouse is over the viewport panel:

| Action | Input |
|--------|-------|
| **Orbit** (rotate around pivot) | MMB drag |
| **Pan** (move pivot) | Shift + MMB drag |
| **Dolly** (zoom in/out) | Scroll wheel |

## Building

```bash
cargo build
cargo run
```

## CSS

Install tailwind cli:

```bash
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 && chmod +x tailwindcss-linux-x64
```

Run in watch mode:

```bash
./tailwindcss-linux-x64 -i assets/tailwind.css -o assets/tailwind-compiled.css --watch
```

## Documentation

- [VIEWPORT_PLUGIN_QUICKSTART.md](./VIEWPORT_PLUGIN_QUICKSTART.md) - Quick start guide
- [VIEWPORT_PLUGINS.md](./VIEWPORT_PLUGINS.md) - Full plugin development guide
- [VIEWPORT_ARCHITECTURE.md](./VIEWPORT_ARCHITECTURE.md) - Architecture diagrams
- [DONE.md](./DONE.md) - Current implementation status

## Project Status

✅ Core architecture complete  
🚧 Viewport providers need IPC implementation  
📝 Documentation complete
