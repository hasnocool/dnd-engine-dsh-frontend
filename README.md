# D&D Engine DSH Frontend

Frontend-only wireframe prototype for the D&D Engine DSH project.

Built with:

- Rust 2024 edition
- Bevy 0.19.1
- Winit 0.30.13

The current scope is intentionally limited to **visual UI wireframes and navigation**. There is no combat engine, character model, persistence, networking, procedural generation, audio system, or gameplay logic.

## Run

```bash
cargo run
```

## Navigation

| Key | Action |
|---|---|
| `↑` / `W` | Previous menu entry |
| `↓` / `S` | Next menu entry |
| `Enter` | Activate highlighted destination |
| `Esc` | Back |
| `1`–`6` | Direct-select numbered menu entries |
| `I` | Game Board → Stats & Inventory |
| `M` | Game Board/Stats → Map |
| `C` | Game Board → Combat |
| `D` | Game Board → Dialog |

All of these actions only change scenes. They do not perform game operations.

## Scene graph

```text
TITLE
  └─Enter→ MAIN MENU
              ├─1→ CHARACTER CREATION ─Enter→ GAME BOARD
              ├─2→ GAME BOARD
              ├─3→ GAME BOARD
              ├─4→ SETTINGS
              ├─5→ CREDITS
              └─6→ END SESSION ─Enter→ TITLE

GAME BOARD
  ├─I→ STATS & INVENTORY
  ├─M→ MAP / WORLD VIEW
  ├─C→ COMBAT
  └─D→ DIALOG / NPC INTERACTION

All secondary scenes ─Esc→ GAME BOARD
SETTINGS / CREDITS / END SESSION ─Esc→ MAIN MENU
MAIN MENU ─Esc→ TITLE
```

## GitHub Actions

No GitHub Actions workflow is included. Local `cargo run`, `cargo check`, and `cargo fmt` are the intended development commands.

## Unicode / terminal-inspired design

The frontend uses box drawing, blocks, arrows, status symbols, and RPG symbols to match the supplied textmode/terminal wireframe direction. Bevy 0.19.1 system font discovery is enabled so the UI can resolve a local monospace font without shipping a font file.
