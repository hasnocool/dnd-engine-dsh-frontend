# UI Wireframes and Navigation Contract

## Screens

1. Title Screen
2. Main Menu
3. Character Creation
4. Main Game Board
5. Stats & Inventory
6. Map / World View
7. Combat / Battle
8. Dialog / NPC Interaction
9. Settings
10. Credits
11. End Session

## Interaction rule

The UI is a presentation prototype. A key press may **navigate to another scene**, but it must not mutate game state or execute game systems.

Examples:

- `C` opens the combat wireframe; it does not resolve an attack.
- `D` opens the dialogue wireframe; it does not alter dialogue state.
- `I` opens the inventory/stats wireframe; it does not equip or consume an item.
- `M` opens the map wireframe; it does not move the player.

## Menu behavior

`↑` / `W` and `↓` / `S` wrap around the current menu. `Enter` activates the selected entry. Number keys provide quick selection when that scene exposes numbered destinations. `Esc` follows the scene's explicit back edge.

## Visual direction

The frontend follows the supplied concept board: dark terminal-style background, cyan/blue outlines, gold selection accents, monospaced typography, dense Unicode framing, dungeon map motifs, and compact information panels.
