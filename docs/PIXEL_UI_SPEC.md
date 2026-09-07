# Fixed 1600×900 CP437 UI Specification

## Rendering contract

The frontend is now authored as a character-cell display rather than as positioned text labels. The logical display is **100 columns × 28 rows**.

| Property | Value |
|---|---:|
| Window | 1600 × 900 px |
| Logical grid | 100 × 28 cells |
| Source glyph | IBM VGA CP437 8 × 16 bitmap |
| Glyph scale | 2 × nearest-neighbour |
| Cell size | 16 × 32 px |
| Drawable grid | 1600 × 896 px |
| Bottom design slack | 4 px |
| Atlas | 32 columns × 8 rows = 256 glyphs |

Every glyph is indexed by its **CP437 byte value**, not by Unicode code point. The renderer uses the IBM VGA 8×16 bitmap repertoire exposed by `neovision-core` 1.1.0, whose font module supplies all 256 glyphs as 16 rows of 8 bits. citeturn23file0

The atlas is generated once at startup from those bitmap bytes and stored as a 256-glyph RGBA texture. Bevy's texture-atlas system provides fixed sub-texture regions, and its sprite atlas API supports indexed rendering of those regions. citeturn324112search0turn324112search1

Nearest-neighbour sampling is mandatory. No font smoothing, font hinting, proportional measurement, or runtime text layout participates in scene composition.

## Screen coordinate system

The origin of the logical grid is the upper-left cell `(0, 0)`.

The conversion to pixels is deterministic:

```text
pixel_x = -800 + column × 16 + 8
pixel_y =  448 - row × 32 - 16 + 2
```

The 2 px vertical adjustment leaves the four-pixel remainder below the 28-row × 32-pixel grid while keeping the visible grid centered consistently on the 1600×900 camera.

## Structural rules

All panel borders are made from CP437 box-drawing characters. All decorative artwork is also stored as cell strings. A panel is a rectangle of cells, not a floating Bevy UI node.

Selection is a cell-background operation: the selected row receives the gold selection background and dark foreground, so its geometry is exactly one cell high with no layout movement.

Status bars are cell glyphs (`█` and `░`) and therefore align exactly with adjacent text and borders.

## CP437 / Unicode policy

The public scene-authoring API accepts Rust `char` strings for readability. Each character is converted through the CP437 repertoire before it reaches the grid. `neovision-core::cp437::from_char` supplies the canonical mapping for characters representable in CP437. citeturn722702search0

Characters outside CP437 use deterministic presentation fallbacks rather than silently invoking a proportional Unicode font. Current examples include sword/weapon symbols mapped to `+`, diamonds mapped to the CP437 diamond, stars mapped to `*`, and check/cross marks mapped to ASCII equivalents.

This distinction is intentional: the underlying rendered display remains a real CP437 cell grid even when scene source uses more readable Unicode notation.

## Scene geometry

The major scenes preserve the previously established visual hierarchy, but their extents are expressed in cells:

| Scene | Primary composition |
|---|---|
| Title | 2 large panels, 43/48 columns, rows 4–22 |
| Main Menu | 40-column menu + 50-column artwork |
| Character Creation | 28 + 45 + 21 column triptych |
| Game Board | 72-column tactical area + 25-column HUD |
| Stats & Inventory | 47 + 46 column information split |
| Map | 75-column world + 19-column legend |
| Inventory | 61-column item list + 33-column detail |
| Combat | 32 + 31 + 31 column triptych |
| Dialog | 28-column portrait + 67-column dialogue |
| Settings | centered 58-column panel |
| Credits | centered 64-column panel |
| End Session | centered 52-column confirmation panel |

Content begins at row `4`; row `25` is the footer divider; rows `26–27` are reserved for navigation/status text.

## Font provenance

The CP437 bitmap source is provided by `neovision-core` 1.1.0. Its font module describes the data as the IBM VGA 8×16 text-mode face and exposes `VGA_8X16: [[u8; 16]; 256]`; the crate itself is MIT-licensed. citeturn23file0turn926534search10

`neovision-core` is used only as a source of the fixed glyph data and CP437 mapping. The frontend does not use its widget system, rendering host, or form navigation layer.

## Performance / determinism notes

The renderer creates one atlas texture and one indexed sprite per non-space cell, plus explicit cell backgrounds only where a panel/selection changes the default background. Scene transitions rebuild the static grid; they do not perform font measurement or text layout.

The design intentionally favors deterministic geometry over minimizing entity count because this repository is a frontend wireframe and navigation prototype. A later optimization can batch the grid into fewer meshes without changing the logical `Grid` representation.

## Scope boundary

Scene transitions and keyboard navigation remain the only functional behaviour. No inventory, character, map, combat, or dialogue data is mutated by these screens.

No GitHub Actions workflows are added by this change.
