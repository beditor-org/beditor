# beditor - Bevy Game Editor

Extensible game editor for Bevy engine built with Dioxus.

## Project status

This project is currently an early proof of concept.

The goal of the first public release is to validate the overall architecture and demonstrate that the chosen approach is technically viable. While the editor already includes a number of core features, it far yet from be considered a production-ready game development tool.

The long-term goal is to gradually evolve it into a full-featured editor for Bevy.

Feedback, ideas, and discussions are greatly appreciated at this stage, as they will help shape the project's future.

## Why a standalone editor?

The Bevy ecosystem has largely gravitated toward in-game editors, which are a perfectly valid approach and offer many advantages.

This project intentionally explores a different direction: a standalone editor communicating with a running game.

I chose this architecture for two reasons.

First, I was personally interested in solving the technical challenges involved in building a separate editor process.

Second, I believe this approach offers several unique advantages, such as clear separation between the game and the editor, independent user interfaces, and the potential for more advanced tooling in the future.

Whether these advantages ultimately outweigh the additional complexity remains to be seen, and exploring that question is one of the motivations behind this project.

## Features

- **Plugin Architecture** - Extensible through plugins
- **Workspaces**
- **Theming** - multiple themes support

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

Requires [Tailwind CLI](https://tailwindcss.com/docs/installation/tailwind-cli). For Linux, you can use the standalone binary:

```bash
curl -#LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 && chmod +x tailwindcss-linux-x64
```

With standalone binay run in watch mode:

```bash
./tailwindcss-linux-x64 -i editor/tailwind.css -o editor/public/main.css --watch
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
