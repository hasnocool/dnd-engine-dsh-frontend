# 1600×900 Pixel UI Specification

## Coordinate system

The frontend uses a fixed 1600×900 design canvas. All scene panels use absolute pixel coordinates measured from the top-left corner.

| Region | Geometry |
|---|---:|
| Canvas | 1600 × 900 px |
| Header | y=10..72 |
| Content | y=72..838 |
| Footer divider | y=850 |
| Footer | y=858..890 |
| Outer left margin | 50–70 px |
| Outer right margin | 50–70 px |
| Standard panel border | 1 px |

## Scene geometry

### Title

- Left panel: `(70,112)` → `690×620`
- Artwork panel: `(790,112)` → `740×620`
- Header title is centered within the fixed 1600×900 canvas.

### Main Menu

- Menu panel: `(70,110)` → `650×625`
- Artwork panel: `(750,110)` → `780×625`
- Menu row: `44 px` high with `14 px` vertical gap.

### Character Creation

- Navigation: `(50,90)` → `390×735`
- Selection/content: `(460,90)` → `700×735`
- Character preview: `(1180,90)` → `370×735`
- Section rows: `40 px` high, spaced by `14 px`.

### Main Game Board

- Tactical viewport: `(50,72)` → `1080×745`
- Character HUD: `(1150,72)` → `400×745`
- The tactical view is intentionally ~70/30 by width.

### Stats & Inventory

- Stats: `(60,72)` → `700×745`
- Inventory: `(790,72)` → `750×745`

### Map

- World viewport: `(50,72)` → `1180×745`
- Legend/sidebar: `(1250,72)` → `300×745`

### Inventory

- Item list: `(50,72)` → `940×745`
- Focused item: `(1010,72)` → `540×745`

### Combat

- Enemy: `(50,72)` → `450×745`
- Command stack: `(520,72)` → `520×745`
- Player HUD: `(1060,72)` → `490×745`

### Dialog

- NPC portrait: `(60,72)` → `560×745`
- Dialogue/choices: `(650,72)` → `900×745`

## Selection contract

Selection is rendered as a 1 px gold border plus a low-alpha gold fill. The pointer is always placed at a fixed 12 px inset from the row's left edge, followed by `[hotkey]` and the label.

Interactive row heights are intentionally fixed so keyboard movement produces no layout shift.

## Typography contract

The prototype uses Bevy `FontSource::Monospace` with `FontSmoothing::None`. Bevy's system-font discovery is enabled in `Cargo.toml`; generic monospace resolution provides the development fallback and keeps the repository asset-free. citeturn642353search0turn445151search0

For a production-quality CP437 presentation, bundle a known fixed-width bitmap/vector font containing the complete CP437 glyph set and switch `text_at()` to `TextFont { font: FontSource::Handle(...) }`. The screen geometry must not change when this happens; only the font source should change.

The CP437 compatibility layer should treat bytes `0x00..0x7F` as the standard ASCII-compatible range and map `0x80..0xFF` to their canonical Unicode code points for rendering. This preserves terminal data semantics while allowing Bevy to render the same glyph vocabulary through Unicode text.

## Visual constraints

- Do not use percentage widths inside scene content panels.
- Do not use flex-based spacing for primary composition.
- Avoid proportional symbols or emoji for structural framing.
- Keep status bars, selection rectangles, and panel borders on integer pixel coordinates.
- All future screens should be added to this same 1600×900 coordinate contract.
