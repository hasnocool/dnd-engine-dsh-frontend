# UI Wireframes and Navigation Contract

## Visual target

The frontend implements the supplied 16:9 concept-board composition as a terminal/textmode-inspired fantasy RPG interface.

Design language:

- near-black blue terminal background
- thin cyan/blue UI structure and panel borders
- gold selection/highlight state
- monospaced system font
- CP437-inspired box drawing and block shading
- Unicode RPG/status glyphs
- dense but readable information hierarchy
- keyboard-first navigation

Bevy 0.19 supports semantic system font selection through `FontSource::Monospace`; the project enables `system_font_discovery` for local font resolution. citeturn219141search0turn219141search12

## Screens

### 1. Title Screen

Three visual zones are used: a left logo/menu panel, a right fantasy skyline/landscape panel, and a persistent top title frame.

Key visual elements:

- `╔═╗ ║ ╚═╝` double-line framing
- D&D RPG ENGINE wordmark
- quote: `"Not all those who wander are lost…"`
- `▶ NEW GAME` highlighted in gold
- mountain/castle-style textmode silhouette
- version/status line

Navigation: `Enter` → Main Menu.

### 2. Character Creation

Three columns match the concept board:

```text
┌───────────────────┐  ┌────────────────────────────┐  ┌────────────┐
│ Create Your Hero  │  │ Select Race                 │  │  Character │
│ ▶ Race            │  │ ┌────────────────────────┐ │  │   Preview  │
│   Class           │  │ │ Human                  │ │  │   ╱██╲    │
│   Background      │  │ │ Elf                    │ │  │  ║██║    │
│   Abilities       │  │ │ Dwarf                  │ │  │   ╲██╱    │
│   Appearance      │  │ │ ...                    │ │  │ STR 14    │
│   Name            │  │ └────────────────────────┘ │  │ DEX 12    │
└───────────────────┘  └────────────────────────────┘  └────────────┘
```

Navigation: `↑/↓`, `W/S`, `1–6` for sections, `7` or `Enter` for the mocked next edge, `Esc` → Main Menu.

### 3. Main Menu

Left: vertically stacked menu. Right: fantasy castle/mountain text art.

Menu:

```text
▶ [1] New Adventure
  [2] Continue
  [3] Load Game
  [4] Settings
  [5] Credits
  [6] Quit
```

### 4. Main Game Board

The game-board composition is deliberately close to the reference image:

- left ~70%: dungeon/tactical view using `█ ░ ▒ ▓ ┌ ┐ └ ┘ ║`, room corridors, chest, enemy and player glyphs
- right ~30%: character HUD
- bottom/context area: current observation

HUD:

```text
ARTHAS  Lv 3  Fighter
HP  ██████████████░░ 28/28
MP  ██████████░░░░░░ 10/10
XP  ███████░░░░░░░░ 34/100
```

Quick destinations:

`I` → Stats & Inventory, `M` → Map, `C` → Combat, `D` → Dialog.

### 5. Stats & Inventory

Two-column information architecture matching the concept image:

- left: STR/DEX/CON/INT/WIS/CHA, saves, skills
- right: weapon, shield, consumables, currency, weight

Example glyph vocabulary:

`⚔` weapon, `◈` equipped gear, `♥` healing, `◇` consumable/material, `†` torch/tool, `★` gold/rare.

### 6. Map / World View

Large overworld viewport plus a compact legend panel.

World vocabulary:

- `@` player
- `⌂` town
- `☠` dungeon
- `▲` forest
- `╱╲` mountains
- `≈≈` water
- `✦` point of interest

### 7. Inventory

Large item table on the left and a focused item detail pane on the right.

The prototype mirrors the reference layout with:

```text
INVENTORY │ EQUIPMENT │ KEY ITEMS │ LOOT

⚔ Longsword       1   5.0 lb   50 gp
◈ Wooden Shield    1   6.0 lb   40 gp
♥ Healing Potion   3   0.5 lb   25 gp
◇ Rations          5   0.2 lb    5 gp
† Torch            4   0.1 lb    2 gp
★ Gold           125          125 gp
```

No item is equipped, dropped, consumed or mutated by the frontend.

### 8. Combat / Battle

Three-column battle scene:

1. enemy presentation
2. command stack
3. player HUD

Commands:

```text
▶ [1] ATTACK
  [2] DEFEND
  [3] SKILLS
  [4] ITEMS
  [5] FLEE
```

A static combat log is shown solely to establish visual hierarchy.

### 9. Dialog / NPC Interaction

Two-zone dialogue layout: NPC portrait on the left, dialogue text/choices on the right.

```text
ELDER SEER

"The path you seek is not easy, young one.
 To the east lies a dungeon filled with ancient
 dangers. Are you prepared?"

▶ 1. I am ready.
  2. Tell me more.
  3. Leave.
```

### 10. Settings

Placeholder navigation for Audio, Display and Controls. These entries only redraw the scene and do not alter application settings.

### 11. Credits

Technology and frontend scope panel.

### 12. End Session

Navigation-only quit confirmation. `Enter` returns to Title; `Esc` returns to Main Menu. The prototype intentionally does not terminate the application from the F10 action.

## Hotkey contract

| Key | Meaning |
|---|---|
| `↑` / `W` | move selection up |
| `↓` / `S` | move selection down |
| `←` / `A` | reserved horizontal navigation |
| `→` / `E` | reserved horizontal navigation |
| `Enter` / `Space` | activate current scene destination |
| `Esc` | follow scene back edge |
| `0`–`9` | numbered scene destination |
| `I` | Stats & Inventory |
| `M` | World Map |
| `C` | Combat |
| `D` | Dialogue |
| `Tab` / `PageDown` | next selection |
| `PageUp` | previous selection |

The D hotkey is intentionally reserved for Dialogue; therefore horizontal-right uses `E` rather than `D`.

## Navigation graph

```text
TITLE
  │ Enter
  ▼
MAIN MENU
  ├─1→ CHARACTER CREATION ─7/Enter→ GAME BOARD
  ├─2→ GAME BOARD
  ├─3→ GAME BOARD
  ├─4→ SETTINGS
  ├─5→ CREDITS
  └─6→ END SESSION ─Enter→ TITLE

GAME BOARD
  ├─1 / I→ STATS & INVENTORY
  ├─2 / M→ MAP
  ├─3 / C→ COMBAT
  └─4 / D→ DIALOG

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

Secondary screens ─Esc→ GAME BOARD
SETTINGS / CREDITS / END SESSION ─Esc→ MAIN MENU
MAIN MENU ─Esc→ TITLE
```

## Glyph contract

The centralized `src/glyphs.rs` palette covers the planned textmode language:

```text
BOXES       ┌ ─ ┐ │ └ ┘ ├ ┤ ┬ ┴ ┼  ╔ ═ ╗ ║ ╚ ╝ ╬
SHADING     ░ ▒ ▓ █ ▀ ▄ ▌ ▐
ARROWS      ↑ ↓ ← → ▲ ▼ ◀ ▶
STATUS      ✓ ✗ ✘ ● ○ ■ □ ◆ ◇ ★ ☆
RPG         ♥ ♠ ♦ ♣ ☠ ⚔ † ‡ ± ∞ · … ␣
WORLD       ⌂ ▲ ╱╲ ≈ ✦ ☼
```

These symbols are presentation vocabulary. They do not imply an executable game mechanic.

## Scope boundary

This repository is intentionally a frontend-only wireframe implementation. Scene transitions are functional; the scenes themselves are not.

No GitHub Actions workflows are included.
