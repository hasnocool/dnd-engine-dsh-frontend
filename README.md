# D&D Engine DSH Frontend

Frontend-only wireframe prototype for the D&D Engine DSH project.

Built with Rust 2024, Bevy 0.19.1 and Winit 0.30.13. Bevy's `system_font_discovery` feature is enabled so the interface can request a system monospace font through `FontSource::Monospace`. Bevy 0.19 added the richer font-selection API, including the `Monospace` semantic font source. citeturn219141search0

## Purpose

This project is a **visual frontend prototype only**. It reproduces the supplied 16:9 concept-board compositions using actual Bevy UI panels, terminal-style borders, block bars, dungeon/map glyphs, menus, status areas and hotkey hints.

There is deliberately no combat resolution, character data mutation, inventory mutation, persistence, networking, procedural generation, audio logic or game rules.

## Scenes

The prototype contains the complete concept-board flow:

1. **Title Screen** — D&D RPG ENGINE logo, quote, menu callout and fantasy ASCII skyline.
2. **Character Creation** — left navigation, race selector, descriptive panel and character stat preview.
3. **Main Menu** — large menu and fantasy castle/mountain artwork.
4. **Main Game Board** — dungeon map on the left, character HUD on the right, contextual actions at the bottom.
5. **Stats & Inventory** — character attributes, saves, skills and item list.
6. **Map / World View** — overworld glyph map plus legend.
7. **Inventory** — item/equipment list and selected-item detail panel.
8. **Combat / Battle** — enemy portrait, combat command stack, player HUD and static combat log.
9. **Dialog / NPC Interaction** — NPC portrait, dialogue copy and numbered response choices.
10. **Settings** — placeholder audio/display/control panels.
11. **Credits** — frontend scope/technology information.
12. **End Session** — navigation-only quit confirmation scene.

## Navigation

| Key | Action |
|---|---|
| `↑` / `W` | Previous menu entry |
| `↓` / `S` | Next menu entry |
| `←` / `A` | Reserved horizontal navigation |
| `→` / `E` | Reserved horizontal navigation |
| `Enter` / `Space` | Activate highlighted destination |
| `Esc` | Follow explicit back edge |
| `0`–`9` | Direct-select numbered menu entries |
| `I` | Open Stats & Inventory from Game Board/Combat |
| `M` | Open World Map |
| `C` | Open Combat |
| `D` | Open Dialogue |
| `Tab` / `PageDown` | Next menu entry |
| `PageUp` | Previous menu entry |

Winit 0.30.13 continues to expose physical `KeyCode` values such as `ArrowUp`, `ArrowDown`, `Digit1`, etc.; the frontend input layer uses Bevy's keyboard input interface while keeping the direct Winit dependency available. citeturn219141search1turn219141search7

## Scene graph

```text
TITLE
  └─Enter→ MAIN MENU
              ├─1→ CHARACTER CREATION ─7/Enter→ GAME BOARD
              ├─2→ GAME BOARD
              ├─3→ GAME BOARD
              ├─4→ SETTINGS
              ├─5→ CREDITS
              └─6→ END SESSION ─Enter→ TITLE

GAME BOARD
  ├─1 / I→ STATS & INVENTORY
  ├─2 / M→ MAP / WORLD VIEW
  ├─3 / C→ COMBAT
  └─4 / D→ DIALOG / NPC INTERACTION

COMBAT
  ├─1→ GAME BOARD
  ├─2→ GAME BOARD
  ├─3→ GAME BOARD
  ├─4→ INVENTORY
  └─5→ GAME BOARD

DIALOG
  ├─1→ GAME BOARD
  ├─2→ GAME BOARD
  └─3→ GAME BOARD

Secondary scenes ─Esc→ GAME BOARD
SETTINGS / CREDITS / END SESSION ─Esc→ MAIN MENU
MAIN MENU ─Esc→ TITLE
```

## CP437 / Unicode textmode palette

The interface intentionally uses Unicode renderings of the classic CP437/textmode vocabulary discussed for the project:

- **Box drawing:** `┌ ─ ┐ │ └ ┘ ├ ┤ ┬ ┴ ┼` and `╔ ═ ╗ ║ ╚ ╝ ╬`
- **Shading:** `░ ▒ ▓ █ ▀ ▄ ▌ ▐`
- **Navigation:** `↑ ↓ ← → ▲ ▼ ◀ ▶`
- **Status:** `✓ ✗ ✘ ● ○ ■ □ ◆ ◇ ★ ☆`
- **RPG:** `♥ ♠ ♦ ♣ ☠ ⚔ † ‡ ± ∞ … ·`
- **World:** `⌂ ▲ ╱╲ ≈ ✦`

`src/glyphs.rs` is the source-of-truth palette so future frontend scenes can reuse the same vocabulary without inventing replacement glyphs.

## 16:9 visual contract

The window defaults to 1600×900 and the screen is structured as:

```text
╔════════════════════════════════════════════════════════════════════════════════════╗
║                              SCENE TITLE                                           ║
╚════════════════════════════════════════════════════════════════════════════════════╝

┌──────────────────────────────┐  ┌──────────────────────────────┐  ┌───────────────┐
│                              │  │                              │  │               │
│         CONTENT              │  │         CONTENT              │  │    HUD /      │
│                              │  │                              │  │    LEGEND     │
└──────────────────────────────┘  └──────────────────────────────┘  └───────────────┘

↑↓ / W S Navigate   ENTER Select   ESC Back   I Inventory   M Map   C Combat   D Dialogue
```

The composition deliberately matches the generated concept board: dark terminal background, cyan/blue structural lines, gold selection state, monospaced text, dense Unicode framing and compact information density.

## Run locally

```bash
cargo check
cargo run
cargo fmt
```

On Linux, Bevy's system-font discovery support may require the `fontconfig` development library. Bevy's 0.19 migration guide specifically notes `libfontconfig1-dev` for Linux builds using system font discovery. citeturn219141search3

## GitHub Actions

No GitHub Actions workflows are included. Development is intentionally local and uses Cargo directly.
