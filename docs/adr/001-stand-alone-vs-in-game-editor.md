# Standalone VS In-Game Editors
## Status: Accepted

## Context
There are 2 major architectures for a game editor:
- **In-game editors.** Editor as part of the game itself. Developers include additional code which, when run, opens the game in editor mode.
- **Standalone editors.** A separate application which can attach to an existing game process or launch it if needed. Examples: Unity, Unreal Engine, Godot.

## Decision
The decision for Beditor is to be a standalone editor. Main motivation:
- access to professional UI tooling without reinventing the wheel: React-like Dioxus framework, HTML/CSS well known to thousands of developers.
- retained mode UI is preferable for tools like editors, unlike immediate mode (egui) which is more natural in Bevy's ECS loop
- Bevy already has BRP (Bevy Remote Protocol) which is designed for working with a `remote` game process.

## Consequences
Development of a standalone editor is associated with solving significant technical challenges and is harder than in-game editors.
These issues include:
- game types serialisation. Editor cannot access game types directly — all data must be serialized over IPC.
- IPC between the game process and the editor process
- capturing the game viewport and streaming it to the editor
- implementation of a downloadable plugin system usable without recompilation is currently pretty limited in Rust given the state of WASM and dynamic linking

Nevertheless, the standalone approach offers multiple advantages:
- editor is all about UI — choosing the standalone approach allows using professional UI frameworks like Dioxus, instead of relying on Bevy's built-in UI capabilities or implementing something from scratch.
- retained mode UI is generally preferable for applications like game editors
- crashes in the game process do not crash the editor
- game designers can work with game assets in the editor without recompiling the whole game
- possible remote debugging of a game running on a different machine
- possible integration with other engines (through implementing additional protocol drivers for them)
