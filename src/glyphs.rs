// src/glyphs.rs
//! Central glyph palette for the D&D terminal-style frontend.
//! These are Unicode renderings of the classic CP437/textmode shapes and
//! commonly used RPG/scene symbols. They are presentation constants only.

pub const CORNER_TL: &str = "┌";
pub const CORNER_TR: &str = "┐";
pub const CORNER_BL: &str = "└";
pub const CORNER_BR: &str = "┘";
pub const HLINE: &str = "─";
pub const VLINE: &str = "│";
pub const DHLINE: &str = "═";
pub const DVLINE: &str = "║";
pub const DCORNER_TL: &str = "╔";
pub const DCORNER_TR: &str = "╗";
pub const DCORNER_BL: &str = "╚";
pub const DCORNER_BR: &str = "╝";
pub const CROSS: &str = "┼";
pub const D_CROSS: &str = "╬";

pub const SHADE_LIGHT: &str = "░";
pub const SHADE_MEDIUM: &str = "▒";
pub const SHADE_DARK: &str = "▓";
pub const BLOCK: &str = "█";
pub const HALF_TOP: &str = "▀";
pub const HALF_BOTTOM: &str = "▄";
pub const HALF_LEFT: &str = "▌";
pub const HALF_RIGHT: &str = "▐";

pub const ARROW_UP: &str = "↑";
pub const ARROW_DOWN: &str = "↓";
pub const ARROW_LEFT: &str = "←";
pub const ARROW_RIGHT: &str = "→";
pub const POINTER_RIGHT: &str = "▶";
pub const POINTER_LEFT: &str = "◀";
pub const POINTER_UP: &str = "▲";
pub const POINTER_DOWN: &str = "▼";

pub const CHECK: &str = "✓";
pub const FAIL: &str = "✗";
pub const CRIT_FAIL: &str = "✘";
pub const ACTIVE: &str = "●";
pub const INACTIVE: &str = "○";
pub const SQUARE: &str = "■";
pub const EMPTY_SQUARE: &str = "□";
pub const DIAMOND: &str = "◆";
pub const EMPTY_DIAMOND: &str = "◇";
pub const STAR: &str = "★";
pub const EMPTY_STAR: &str = "☆";

pub const HEART: &str = "♥";
pub const SPADE: &str = "♠";
pub const CLUB: &str = "♣";
pub const CARD_DIAMOND: &str = "♦";
pub const SKULL: &str = "☠";
pub const SWORDS: &str = "⚔";
pub const DAGGER: &str = "†";
pub const DOUBLE_DAGGER: &str = "‡";
pub const DEGREE: &str = "°";
pub const PLUS_MINUS: &str = "±";
pub const INFINITY: &str = "∞";
pub const MIDDLE_DOT: &str = "·";
pub const ELLIPSIS: &str = "…";
pub const OPEN_BOX: &str = "␣";
pub const SUN: &str = "☼";
pub const HOME: &str = "⌂";
