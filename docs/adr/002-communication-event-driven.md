# Game-Editor Communication and Responsibilities

## Status: Proposed

## Context

Communication between the editor and the game should not rely on classical request-response (RPC). Instead, both sides simply exchange updates without waiting for a reply. This is an **event-driven** approach (also known as fire-and-forget or one-way messaging).

With this approach the editor can display up-to-date entity/component lists, and the game can react to editor commands — all without either side blocking on a response.

**Open question:** Are there cases where request-response is genuinely needed? Candidates include initial state sync on connect, and operations that can fail with a meaningful error (e.g. loading a scene file). For now we proceed with the event-driven approach and revisit if such cases arise.

## Decision

The responsibility of the **editor** is to:
- Send commands to the game
- Receive updates from the game and store them in Dioxus signals

The editor does **not** request entity or component lists explicitly — the game sends them on its own initiative.

The responsibility of the **game** is to:
- Track game world state changes and push updates to the editor
- Listen to editor commands and apply them to the game world

## Consequences

- No need for correlation IDs, timeouts, or retry logic in the protocol
- There is no explicit confirmation that a command was applied; the editor infers success by observing the next state update from the game
- Error reporting requires a dedicated message type (e.g. `command_rejected`) rather than an inline response


