// src/pixel_ui.rs
//! Deterministic 1600x900 textmode-inspired frontend renderer.
//!
//! The original `ui.rs` remains available as a reference implementation. This
//! renderer uses absolute pixel coordinates so panel proportions, spacing,
//! selection boxes, and glyph placement do not depend on flex-flow measurement.
//!
//! Font strategy:
//! - Prefer a fixed-width monospace face with complete box-drawing coverage.
//! - Render with `FontSmoothing::None` for crisp terminal/pixel presentation.
//! - `FontSource::Monospace` is used here so the prototype remains dependency- and
//!   asset-free. A production build can swap this single source to a bundled
//!   IBM VGA/CP437 bitmap font handle without changing the screen geometry.

use bevy::prelude::*;

use crate::theme::Theme;

pub const DESIGN_WIDTH: f32 = 1600.0;
pub const DESIGN_HEIGHT: f32 = 900.0;
const FONT_SIZE: f32 = 18.0;
const SMALL_FONT: f32 = 14.0;
const TINY_FONT: f32 = 12.0;
const HEADER_FONT: f32 = 26.0;
const SCREEN_TOP: f32 = 72.0;
const SCREEN_BOTTOM: f32 = 838.0;
const FOOTER_Y: f32 = 850.0;

#[derive(Component)]
pub struct UiRoot;

#[derive(Resource)]
pub struct CurrentScene {
    pub scene: Scene,
}

impl Default for CurrentScene {
    fn default() -> Self {
        Self { scene: Scene::Title }
    }
}

#[derive(Resource, Default)]
pub struct NavigationState {
    pub selected: usize,
}

impl NavigationState {
    pub fn reset(&mut self, _scene: Scene) {
        self.selected = 0;
    }

    pub fn move_up(&mut self, scene: Scene) -> bool {
        let len = scene.menu_items().len();
        if len == 0 {
            return false;
        }
        self.selected = if self.selected == 0 { len - 1 } else { self.selected - 1 };
        true
    }

    pub fn move_down(&mut self, scene: Scene) -> bool {
        let len = scene.menu_items().len();
        if len == 0 {
            return false;
        }
        self.selected = (self.selected + 1) % len;
        true
    }

    pub fn selected_target(&self, scene: Scene) -> Option<Scene> {
        scene
            .menu_items()
            .get(self.selected)
            .map(|item| item.target)
            .or_else(|| match scene {
                Scene::Title => Some(Scene::MainMenu),
                Scene::GameBoard
                | Scene::StatsInventory
                | Scene::Inventory
                | Scene::Map
                | Scene::Combat
                | Scene::Dialog => Some(Scene::GameBoard),
                Scene::CharacterCreation => Some(Scene::GameBoard),
                Scene::Settings | Scene::Credits => Some(Scene::MainMenu),
                Scene::Exit => Some(Scene::Title),
                Scene::MainMenu => None,
            })
    }

    pub fn number_target(&self, scene: Scene, number: usize) -> Option<Scene> {
        let key = number.to_string();
        scene
            .menu_items()
            .iter()
            .find(|item| item.hotkey == key)
            .map(|item| item.target)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scene {
    Title,
    MainMenu,
    CharacterCreation,
    GameBoard,
    StatsInventory,
    Map,
    Inventory,
    Combat,
    Dialog,
    Settings,
    Credits,
    Exit,
}

#[derive(Clone, Copy)]
pub struct NavItem {
    pub label: &'static str,
    pub target: Scene,
    pub hotkey: &'static str,
}

impl Scene {
    pub fn title(self) -> &'static str {
        match self {
            Scene::Title => "D&D RPG ENGINE",
            Scene::MainMenu => "MAIN MENU",
            Scene::CharacterCreation => "CREATE YOUR HERO",
            Scene::GameBoard => "MAIN GAME BOARD",
            Scene::StatsInventory => "STATS & INVENTORY",
            Scene::Map => "MAP / WORLD VIEW",
            Scene::Inventory => "INVENTORY",
            Scene::Combat => "COMBAT / BATTLE",
            Scene::Dialog => "DIALOG / NPC INTERACTION",
            Scene::Settings => "SETTINGS",
            Scene::Credits => "CREDITS",
            Scene::Exit => "END SESSION",
        }
    }

    pub fn back(self) -> Option<Self> {
        match self {
            Scene::Title => None,
            Scene::MainMenu => Some(Scene::Title),
            Scene::CharacterCreation => Some(Scene::MainMenu),
            Scene::GameBoard => Some(Scene::MainMenu),
            Scene::StatsInventory
            | Scene::Map
            | Scene::Inventory
            | Scene::Combat
            | Scene::Dialog => Some(Scene::GameBoard),
            Scene::Settings | Scene::Credits | Scene::Exit => Some(Scene::MainMenu),
        }
    }

    pub fn menu_items(self) -> &'static [NavItem] {
        const MAIN: &[NavItem] = &[
            NavItem { label: "New Adventure", target: Scene::CharacterCreation, hotkey: "1" },
            NavItem { label: "Continue", target: Scene::GameBoard, hotkey: "2" },
            NavItem { label: "Load Game", target: Scene::GameBoard, hotkey: "3" },
            NavItem { label: "Settings", target: Scene::Settings, hotkey: "4" },
            NavItem { label: "Credits", target: Scene::Credits, hotkey: "5" },
            NavItem { label: "Quit", target: Scene::Exit, hotkey: "6" },
        ];
        const CREATION: &[NavItem] = &[
            NavItem { label: "Race", target: Scene::CharacterCreation, hotkey: "1" },
            NavItem { label: "Class", target: Scene::CharacterCreation, hotkey: "2" },
            NavItem { label: "Background", target: Scene::CharacterCreation, hotkey: "3" },
            NavItem { label: "Abilities", target: Scene::CharacterCreation, hotkey: "4" },
            NavItem { label: "Appearance", target: Scene::CharacterCreation, hotkey: "5" },
            NavItem { label: "Name", target: Scene::CharacterCreation, hotkey: "6" },
            NavItem { label: "Next", target: Scene::GameBoard, hotkey: "7" },
        ];
        const BOARD: &[NavItem] = &[
            NavItem { label: "Inventory / Stats", target: Scene::StatsInventory, hotkey: "1" },
            NavItem { label: "World Map", target: Scene::Map, hotkey: "2" },
            NavItem { label: "Combat", target: Scene::Combat, hotkey: "3" },
            NavItem { label: "Dialogue", target: Scene::Dialog, hotkey: "4" },
        ];
        const SETTINGS: &[NavItem] = &[
            NavItem { label: "Audio", target: Scene::Settings, hotkey: "1" },
            NavItem { label: "Display", target: Scene::Settings, hotkey: "2" },
            NavItem { label: "Controls", target: Scene::Settings, hotkey: "3" },
            NavItem { label: "Back", target: Scene::MainMenu, hotkey: "0" },
        ];
        const COMBAT: &[NavItem] = &[
            NavItem { label: "Attack", target: Scene::GameBoard, hotkey: "1" },
            NavItem { label: "Defend", target: Scene::GameBoard, hotkey: "2" },
            NavItem { label: "Skills", target: Scene::GameBoard, hotkey: "3" },
            NavItem { label: "Items", target: Scene::Inventory, hotkey: "4" },
            NavItem { label: "Flee", target: Scene::GameBoard, hotkey: "5" },
        ];
        const DIALOG: &[NavItem] = &[
            NavItem { label: "I am ready.", target: Scene::GameBoard, hotkey: "1" },
            NavItem { label: "Tell me more.", target: Scene::GameBoard, hotkey: "2" },
            NavItem { label: "Leave.", target: Scene::GameBoard, hotkey: "3" },
        ];
        match self {
            Scene::MainMenu => MAIN,
            Scene::CharacterCreation => CREATION,
            Scene::GameBoard => BOARD,
            Scene::Settings => SETTINGS,
            Scene::Combat => COMBAT,
            Scene::Dialog => DIALOG,
            _ => &[],
        }
    }
}

pub fn build_scene(
    commands: &mut Commands,
    theme: &Theme,
    _assets: &AssetServer,
    scene: Scene,
    nav: &NavigationState,
) {
    commands
        .spawn((
            Node {
                width: Val::Px(DESIGN_WIDTH),
                height: Val::Px(DESIGN_HEIGHT),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(theme.background),
            UiRoot,
        ))
        .with_children(|page| {
            frame(page, theme, scene.title());
            match scene {
                Scene::Title => title(page, theme),
                Scene::MainMenu => main_menu(page, theme, nav.selected),
                Scene::CharacterCreation => creation(page, theme, nav.selected),
                Scene::GameBoard => board(page, theme, nav.selected),
                Scene::StatsInventory => stats(page, theme),
                Scene::Map => map(page, theme),
                Scene::Inventory => inventory(page, theme),
                Scene::Combat => combat(page, theme, nav.selected),
                Scene::Dialog => dialog(page, theme, nav.selected),
                Scene::Settings => settings(page, theme, nav.selected),
                Scene::Credits => credits(page, theme),
                Scene::Exit => exit(page, theme),
            }
            footer(page, theme);
        });
}

fn frame(parent: &mut ChildSpawner, t: &Theme, title: &str) {
    text_at(parent, t, 28.0, 10.0, 20.0, "╔════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗", t.accent);
    text_at(parent, t, 28.0, 28.0, HEADER_FONT, title, t.text);
    text_at(parent, t, 28.0, 52.0, 20.0, "╚════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝", t.accent);
}

fn footer(parent: &mut ChildSpawner, t: &Theme) {
    line(parent, t, 48.0, FOOTER_Y, 1504.0, t.border);
    text_at(parent, t, 48.0, FOOTER_Y + 8.0, SMALL_FONT, "↑↓ / W S   NAVIGATE    ENTER / SPACE   SELECT    0-9   QUICK SELECT    ESC   BACK", t.dim);
    text_at(parent, t, 48.0, FOOTER_Y + 28.0, TINY_FONT, "I Inventory   M Map   C Combat   D Dialogue   A/E Reserved Horizontal Navigation", t.dim);
    text_at(parent, t, 1190.0, FOOTER_Y + 28.0, TINY_FONT, "1600×900  ·  TEXTMODE FRONTEND  ·  NAVIGATION ONLY", t.dim);
}

fn title(parent: &mut ChildSpawner, t: &Theme) {
    panel(parent, t, 70.0, 112.0, 690.0, 620.0);
    panel(parent, t, 790.0, 112.0, 740.0, 620.0);

    text_at(parent, t, 112.0, 152.0, 20.0, "╔══════════════════╗", t.primary);
    text_at(parent, t, 112.0, 176.0, 30.0, "║    D&D RPG     ║", t.text);
    text_at(parent, t, 112.0, 212.0, 30.0, "║     ENGINE     ║", t.text);
    text_at(parent, t, 112.0, 248.0, 20.0, "╚══════════════════╝", t.primary);
    text_at(parent, t, 112.0, 302.0, FONT_SIZE, "\"Not all those who wander are lost…\"", t.text);
    text_at(parent, t, 112.0, 346.0, SMALL_FONT, "A TERMINAL-STYLE FANTASY ADVENTURE", t.dim);
    selected_line(parent, t, 112.0, 410.0, 576.0, 42.0, true, "1", "NEW GAME");
    selected_line(parent, t, 112.0, 462.0, 576.0, 42.0, false, "2", "CONTINUE");
    selected_line(parent, t, 112.0, 514.0, 576.0, 42.0, false, "3", "SETTINGS");
    selected_line(parent, t, 112.0, 566.0, 576.0, 42.0, false, "4", "QUIT");

    text_at(parent, t, 960.0, 152.0, 20.0, "                         ▲", t.primary);
    text_at(parent, t, 960.0, 176.0, FONT_SIZE, "                        ╱ ╲", t.primary);
    text_at(parent, t, 910.0, 200.0, FONT_SIZE, "                  ╱╲   ╱███╲   ╱╲", t.primary);
    text_at(parent, t, 900.0, 224.0, FONT_SIZE, "                 ╱██╲ ╱█████╲ ╱██╲", t.primary);
    text_at(parent, t, 890.0, 248.0, FONT_SIZE, "                ║████║║█████║║████║", t.primary);
    text_at(parent, t, 1040.0, 272.0, FONT_SIZE, "               ═══╩═══", t.dim);
    text_at(parent, t, 1000.0, 350.0, 20.0, "★  ADVENTURE AWAITS  ★", t.accent);
    text_at(parent, t, 940.0, 388.0, FONT_SIZE, "⚔  EXPLORE · FIGHT · DISCOVER · SURVIVE", t.text);
}

fn main_menu(parent: &mut ChildSpawner, t: &Theme, selected: usize) {
    panel(parent, t, 70.0, 110.0, 650.0, 625.0);
    panel(parent, t, 750.0, 110.0, 780.0, 625.0);

    text_at(parent, t, 106.0, 144.0, 20.0, "MAIN MENU", t.accent);
    for (i, (k, label)) in [
        ("1", "New Adventure"),
        ("2", "Continue"),
        ("3", "Load Game"),
        ("4", "Settings"),
        ("5", "Credits"),
        ("6", "Quit"),
    ]
    .iter()
    .enumerate()
    {
        selected_line(parent, t, 106.0, 200.0 + i as f32 * 58.0, 570.0, 44.0, selected == i, k, label);
    }

    text_at(parent, t, 988.0, 170.0, 18.0, "                ▲", t.primary);
    text_at(parent, t, 918.0, 194.0, 18.0, "          ╱╲   ╱██╲   ╱╲", t.primary);
    text_at(parent, t, 900.0, 218.0, 18.0, "         ║████║║████║║████║", t.primary);
    text_at(parent, t, 978.0, 260.0, 18.0, "           ═══╩═══", t.dim);
    text_at(parent, t, 990.0, 342.0, 20.0, "★  ADVENTURE AWAITS  ★", t.accent);
    text_at(parent, t, 900.0, 390.0, FONT_SIZE, "A realm built from glyphs, grids, and legends.", t.text);
}

fn creation(parent: &mut ChildSpawner, t: &Theme, selected: usize) {
    panel(parent, t, 50.0, 90.0, 390.0, 735.0);
    panel(parent, t, 460.0, 90.0, 700.0, 735.0);
    panel(parent, t, 1180.0, 90.0, 370.0, 735.0);

    text_at(parent, t, 76.0, 122.0, 20.0, "CREATE YOUR HERO", t.accent);
    for (i, label) in ["Race", "Class", "Background", "Abilities", "Appearance", "Name"].iter().enumerate() {
        selected_line(parent, t, 76.0, 178.0 + i as f32 * 54.0, 338.0, 40.0, selected == i, &(i + 1).to_string(), label);
    }
    selected_line(parent, t, 76.0, 548.0, 338.0, 44.0, selected == 6, "7", "NEXT → ENTER");

    text_at(parent, t, 492.0, 122.0, 20.0, "SELECT RACE", t.accent);
    boxed_text(parent, t, 492.0, 168.0, 632.0, 300.0);
    for (i, label) in ["Human", "Elf", "Dwarf", "Halfling", "Dragonborn", "Tiefling", "Half-Elf", "Half-Orc"].iter().enumerate() {
        text_at(parent, t, 520.0, 196.0 + i as f32 * 30.0, SMALL_FONT, &format!("│ {:<48} │", label), t.text);
    }
    text_at(parent, t, 520.0, 500.0, 20.0, "HUMAN", t.accent);
    text_at(parent, t, 520.0, 532.0, SMALL_FONT, "Versatile and ambitious.", t.text);
    text_at(parent, t, 520.0, 558.0, SMALL_FONT, "+1 STR   +1 CON", t.success);

    text_at(parent, t, 1208.0, 122.0, 18.0, "CHARACTER PREVIEW", t.accent);
    text_at(parent, t, 1320.0, 176.0, FONT_SIZE, "╔══════╗", t.primary);
    text_at(parent, t, 1320.0, 200.0, FONT_SIZE, "║ ╱██╲ ║", t.primary);
    text_at(parent, t, 1320.0, 224.0, FONT_SIZE, "║║████║║", t.primary);
    text_at(parent, t, 1320.0, 248.0, FONT_SIZE, "║ ╲██╱ ║", t.primary);
    text_at(parent, t, 1320.0, 272.0, FONT_SIZE, "╚══════╝", t.primary);
    for (i, s) in ["STR 14 (+2)", "DEX 12 (+1)", "CON 13 (+1)", "INT 10 (+0)", "WIS 11 (+0)", "CHA 08 (-1)"].iter().enumerate() {
        text_at(parent, t, 1210.0, 340.0 + i as f32 * 30.0, SMALL_FONT, s, t.text);
    }
}

fn board(parent: &mut ChildSpawner, t: &Theme, selected: usize) {
    panel(parent, t, 50.0, SCREEN_TOP, 1080.0, 745.0);
    panel(parent, t, 1150.0, SCREEN_TOP, 400.0, 745.0);
    text_at(parent, t, 78.0, 102.0, 18.0, "OLD MINES — LEVEL 2", t.accent);

    let dungeon = [
        "╔════════════════════════════════════════════════════════════════════════════════════════════════╗",
        "║░░░░░░░░░░░░░░░░░░░░░░║                  ║░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░║",
        "║░  ┌──────────────┐  ░║   ┌──────────┐   ║░  ┌──────────────────────┐   ░░░░░░░░░║",
        "║░  │      ☠       │  ░╚═══╡    ▲     ╞═══╝░  │         ░░░░░░░░░░░░░ │   ░░░░░░░║",
        "║░  │              └───────┐          └───────┘      ░░░░░░░░░░░░░░ │   ░░░░░░║",
        "║░  │     ████               │                         ░░░░░░░░░░░░░░ │   ░░░░░░░║",
        "║░  │     █ @█     ╔══════╗  │    ╔═══════════════════╗   ░░░░░░░░░░░  │   ░░░░░║",
        "║░  │      ██      ║ CHEST║  └────║    ANCIENT HALL   ║──────┐  ░░░░ │   ░░░░║",
        "║░  └──────────────╚══════╝       ╚═══════════════════╝      │  ░░░░ │   ░░░░║",
        "║░                 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│  ░░░░ │   ░░║",
        "║░  ╔════════════════╗                         ┌──────────────┘  ░░░░░░░░░░░░║",
        "║░  ║  OLD TUNNEL    ║═════════════════════════╝                  ░░░░░░░░░░║",
        "║░  ╚════════════════╝             ☠ goblin                  ░░░░░░░░░░░░░░║",
        "║░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░║",
        "╚════════════════════════════════════════════════════════════════════════════════════════════════╝",
    ];
    for (i, row) in dungeon.iter().enumerate() {
        text_at(parent, t, 78.0, 140.0 + i as f32 * 28.0, 15.0, row, t.text);
    }
    text_at(parent, t, 78.0, 610.0, 16.0, "▶ You see a goblin (Lv 2)", t.warning);
    text_at(parent, t, 78.0, 646.0, SMALL_FONT, "The room is cold. Water drips somewhere beyond the eastern wall.", t.dim);

    text_at(parent, t, 1182.0, 102.0, 18.0, "ARTHAS  Lv 3  FIGHTER", t.text);
    bar(parent, t, 1182.0, 150.0, "HP", 28, 28, t.success);
    bar(parent, t, 1182.0, 192.0, "MP", 10, 10, t.primary);
    bar(parent, t, 1182.0, 234.0, "XP", 34, 100, t.accent);
    text_at(parent, t, 1182.0, 292.0, SMALL_FONT, "QUICK DESTINATIONS", t.accent);
    for (i, (k, label)) in [
        ("1", "Inventory / Stats"),
        ("2", "World Map"),
        ("3", "Combat"),
        ("4", "Dialogue"),
    ]
    .iter()
    .enumerate()
    {
        selected_line(parent, t, 1182.0, 328.0 + i as f32 * 52.0, 332.0, 40.0, selected == i, k, label);
    }
    text_at(parent, t, 1182.0, 572.0, TINY_FONT, "@ = PLAYER   ☠ = ENEMY   ▲ = LANDMARK", t.dim);
    text_at(parent, t, 1182.0, 600.0, TINY_FONT, "░▒▓█ = DEPTH / COVER / WALL MASS", t.dim);
}

fn stats(parent: &mut ChildSpawner, t: &Theme) {
    panel(parent, t, 60.0, SCREEN_TOP, 700.0, 745.0);
    panel(parent, t, 790.0, SCREEN_TOP, 750.0, 745.0);
    text_at(parent, t, 88.0, 102.0, 18.0, "CHARACTER STATS", t.accent);
    for (i, s) in [
        "STR 14 (+2)",
        "DEX 12 (+1)",
        "CON 13 (+1)",
        "INT 10 (+0)",
        "WIS 11 (+0)",
        "CHA 08 (-1)",
        "────────────────────────────────────────",
        "Saving Throws  Fort +4  Ref +2  Will +1",
        "Skills         Athletics +4  Stealth +3",
        "                Perception +1",
    ]
    .iter()
    .enumerate()
    {
        text_at(parent, t, 96.0, 152.0 + i as f32 * 40.0, SMALL_FONT, s, t.text);
    }
    text_at(parent, t, 818.0, 102.0, 18.0, "INVENTORY", t.accent);
    for (i, s) in [
        "⚔ Longsword                 (equipped)",
        "◈ Wooden Shield             (equipped)",
        "♥ Healing Potion x3",
        "◇ Rations x5",
        "† Torch x4",
        "★ Gold 125",
    ]
    .iter()
    .enumerate()
    {
        text_at(parent, t, 826.0, 152.0 + i as f32 * 44.0, SMALL_FONT, s, t.text);
    }
    line(parent, t, 826.0, 446.0, 650.0, t.border);
    text_at(parent, t, 826.0, 472.0, SMALL_FONT, "Weight: 12 / 50 lb", t.dim);
    text_at(parent, t, 826.0, 526.0, 18.0, "STATUS", t.accent);
    text_at(parent, t, 826.0, 566.0, SMALL_FONT, "● Healthy", t.success);
    text_at(parent, t, 826.0, 606.0, SMALL_FONT, "○ Poisoned", t.dim);
}

fn map(parent: &mut ChildSpawner, t: &Theme) {
    panel(parent, t, 50.0, SCREEN_TOP, 1180.0, 745.0);
    panel(parent, t, 1250.0, SCREEN_TOP, 300.0, 745.0);
    text_at(parent, t, 78.0, 102.0, 18.0, "WORLD MAP", t.accent);
    let rows = [
        "╱╲        ▲▲▲▲▲▲▲             ╱╲╱╲╱╲         ▲▲▲▲▲      ╱╲",
        "   ╱╲   ▲▲▲▲▲▲▲▲▲      ╱╲        ╱╲      ▲▲▲▲▲▲▲▲▲      ╱╲",
        "≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈",
        "≈≈      ⌂ VALLEN        ────────────────      ✦ RUINS     ≈≈",
        "≈≈             ────────       @        ────────             ≈≈",
        "≈≈    ▲▲▲▲▲▲   │            ╱╲╱╲             │   ☠ DUNGEON ≈≈",
        "≈≈    ▲▲▲▲▲▲   │        ╱╲  ║██║  ╱╲        │              ≈≈",
        "≈≈             └─────────╢████╞─────────────┘              ≈≈",
        "≈≈                   ╲╲     │     ╱╱                        ≈≈",
        "≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈",
        "         ╱╲╱╲╱╲           ▲▲▲▲▲▲▲▲▲          ╱╲╱╲╱╲",
        "      ╱╲         ╲╲     ▲▲▲▲▲▲▲▲▲▲▲      ╱╲         ╲╲",
    ];
    for (i, row) in rows.iter().enumerate() {
        text_at(parent, t, 78.0, 152.0 + i as f32 * 42.0, 16.0, row, t.text);
    }
    text_at(parent, t, 1280.0, 102.0, 18.0, "LEGEND", t.accent);
    for (i, (glyph, label)) in [
        ("@", "Player"),
        ("⌂", "Town"),
        ("☠", "Dungeon"),
        ("▲", "Forest / Landmark"),
        ("╱╲", "Mountains"),
        ("≈", "Water"),
        ("✦", "Point of Interest"),
    ]
    .iter()
    .enumerate()
    {
        text_at(parent, t, 1280.0, 164.0 + i as f32 * 44.0, SMALL_FONT, &format!("{:>3}   {}", glyph, label), t.text);
    }
    line(parent, t, 1280.0, 500.0, 235.0, t.border);
    text_at(parent, t, 1280.0, 530.0, TINY_FONT, "REGION", t.dim);
    text_at(parent, t, 1280.0, 554.0, SMALL_FONT, "Northern Marches", t.text);
    text_at(parent, t, 1280.0, 598.0, TINY_FONT, "SCALE", t.dim);
    text_at(parent, t, 1280.0, 622.0, SMALL_FONT, "1 hex ≈ 1 mile", t.text);
}

fn inventory(parent: &mut ChildSpawner, t: &Theme) {
    panel(parent, t, 50.0, SCREEN_TOP, 940.0, 745.0);
    panel(parent, t, 1010.0, SCREEN_TOP, 540.0, 745.0);
    text_at(parent, t, 78.0, 102.0, 18.0, "INVENTORY │ EQUIPMENT │ KEY ITEMS │ LOOT", t.accent);
    for (i, s) in [
        "⚔ Longsword       1   5.0 lb   50 gp",
        "◈ Wooden Shield    1   6.0 lb   40 gp",
        "♥ Healing Potion   3   0.5 lb   25 gp",
        "◇ Rations          5   0.2 lb    5 gp",
        "† Torch             4   0.1 lb    2 gp",
        "★ Gold            125              ",
    ]
    .iter()
    .enumerate()
    {
        selected_line(parent, t, 78.0, 162.0 + i as f32 * 52.0, 880.0, 40.0, i == 0, " ", s);
    }
    text_at(parent, t, 1038.0, 102.0, 18.0, "FOCUSED ITEM", t.accent);
    text_at(parent, t, 1040.0, 166.0, 22.0, "⚔ LONGSWORD", t.text);
    text_at(parent, t, 1040.0, 214.0, SMALL_FONT, "A well-crafted longsword.", t.text);
    text_at(parent, t, 1040.0, 260.0, SMALL_FONT, "Damage: 1d8+2", t.danger);
    text_at(parent, t, 1040.0, 300.0, SMALL_FONT, "Weight: 5.0 lb", t.text);
    text_at(parent, t, 1040.0, 340.0, SMALL_FONT, "Value: 50 gp", t.accent);
    line(parent, t, 1040.0, 390.0, 450.0, t.border);
    text_at(parent, t, 1040.0, 420.0, TINY_FONT, "FRONTEND STATE", t.dim);
    text_at(parent, t, 1040.0, 450.0, SMALL_FONT, "No item mutations in wireframe mode.", t.dim);
}

fn combat(parent: &mut ChildSpawner, t: &Theme, selected: usize) {
    panel(parent, t, 50.0, SCREEN_TOP, 450.0, 745.0);
    panel(parent, t, 520.0, SCREEN_TOP, 520.0, 745.0);
    panel(parent, t, 1060.0, SCREEN_TOP, 490.0, 745.0);
    text_at(parent, t, 78.0, 102.0, 18.0, "ENEMY", t.danger);
    text_at(parent, t, 190.0, 180.0, 18.0, "╔════════╗", t.danger);
    text_at(parent, t, 190.0, 208.0, 18.0, "║  ☠    ║", t.danger);
    text_at(parent, t, 190.0, 236.0, 18.0, "║ GOBLIN ║", t.danger);
    text_at(parent, t, 190.0, 264.0, 18.0, "╚════════╝", t.danger);
    bar(parent, t, 92.0, 344.0, "HP", 16, 16, t.danger);
    text_at(parent, t, 78.0, 402.0, SMALL_FONT, "Goblin Scout", t.text);
    text_at(parent, t, 78.0, 432.0, SMALL_FONT, "AC 14   Initiative 12", t.dim);

    text_at(parent, t, 550.0, 102.0, 18.0, "COMMAND", t.accent);
    for (i, label) in ["Attack", "Defend", "Skills", "Items", "Flee"].iter().enumerate() {
        selected_line(parent, t, 550.0, 158.0 + i as f32 * 56.0, 460.0, 44.0, selected == i, &(i + 1).to_string(), label);
    }
    line(parent, t, 550.0, 468.0, 460.0, t.border);
    text_at(parent, t, 550.0, 500.0, SMALL_FONT, "COMBAT LOG", t.accent);
    for (i, s) in [
        "> A goblin emerges from the dark.",
        "> Initiative order established.",
        "> Waiting for player selection…",
    ]
    .iter()
    .enumerate()
    {
        text_at(parent, t, 550.0, 536.0 + i as f32 * 32.0, TINY_FONT, s, t.dim);
    }

    text_at(parent, t, 1090.0, 102.0, 18.0, "PLAYER HUD", t.accent);
    text_at(parent, t, 1090.0, 152.0, 18.0, "ARTHAS  Lv 3  FIGHTER", t.text);
    bar(parent, t, 1090.0, 196.0, "HP", 28, 28, t.success);
    bar(parent, t, 1090.0, 238.0, "MP", 10, 10, t.primary);
    text_at(parent, t, 1090.0, 298.0, SMALL_FONT, "Weapon: Longsword", t.text);
    text_at(parent, t, 1090.0, 332.0, SMALL_FONT, "Defense: Shield", t.text);
    text_at(parent, t, 1090.0, 366.0, SMALL_FONT, "Status: ● Healthy", t.success);
}

fn dialog(parent: &mut ChildSpawner, t: &Theme, selected: usize) {
    panel(parent, t, 60.0, SCREEN_TOP, 560.0, 745.0);
    panel(parent, t, 650.0, SCREEN_TOP, 900.0, 745.0);
    text_at(parent, t, 88.0, 102.0, 18.0, "ELDER SEER", t.accent);
    text_at(parent, t, 224.0, 176.0, 18.0, "╔════════════╗", t.primary);
    text_at(parent, t, 224.0, 204.0, 18.0, "║    ☼       ║", t.primary);
    text_at(parent, t, 224.0, 232.0, 18.0, "║   ╱██╲     ║", t.primary);
    text_at(parent, t, 224.0, 260.0, 18.0, "║  ║████║    ║", t.primary);
    text_at(parent, t, 224.0, 288.0, 18.0, "║   ╲██╱     ║", t.primary);
    text_at(parent, t, 224.0, 316.0, 18.0, "╚════════════╝", t.primary);

    text_at(parent, t, 680.0, 102.0, 18.0, "ELDER SEER", t.accent);
    text_at(parent, t, 680.0, 166.0, FONT_SIZE, "\"The path you seek is not easy, young one.\"", t.text);
    text_at(parent, t, 680.0, 202.0, FONT_SIZE, "\"To the east lies a dungeon filled with ancient\"", t.text);
    text_at(parent, t, 680.0, 238.0, FONT_SIZE, "\"dangers. Are you prepared?\"", t.text);
    for (i, label) in ["I am ready.", "Tell me more.", "Leave."].iter().enumerate() {
        selected_line(parent, t, 680.0, 360.0 + i as f32 * 58.0, 790.0, 44.0, selected == i, &(i + 1).to_string(), label);
    }
}

fn settings(parent: &mut ChildSpawner, t: &Theme, selected: usize) {
    panel(parent, t, 430.0, 120.0, 740.0, 620.0);
    text_at(parent, t, 468.0, 152.0, 20.0, "SETTINGS", t.accent);
    for (i, label) in ["Audio", "Display", "Controls", "Back"].iter().enumerate() {
        selected_line(parent, t, 468.0, 222.0 + i as f32 * 62.0, 660.0, 46.0, selected == i, if i == 3 { "0" } else { &(i + 1).to_string() }, label);
    }
    text_at(parent, t, 468.0, 520.0, SMALL_FONT, "These controls are visual placeholders only.", t.dim);
}

fn credits(parent: &mut ChildSpawner, t: &Theme) {
    panel(parent, t, 360.0, 118.0, 880.0, 625.0);
    text_at(parent, t, 398.0, 152.0, 20.0, "CREDITS", t.accent);
    text_at(parent, t, 430.0, 222.0, FONT_SIZE, "D&D RPG ENGINE", t.text);
    text_at(parent, t, 430.0, 264.0, SMALL_FONT, "Rust · Bevy · Winit", t.primary);
    text_at(parent, t, 430.0, 310.0, SMALL_FONT, "Frontend-only wireframe / navigation prototype", t.text);
    text_at(parent, t, 430.0, 356.0, SMALL_FONT, "CP437-inspired box drawing + Unicode RPG vocabulary", t.text);
    text_at(parent, t, 430.0, 402.0, SMALL_FONT, "Designed around a deterministic 1600×900 canvas", t.text);
    text_at(parent, t, 430.0, 470.0, TINY_FONT, "No gameplay systems are implemented in this prototype.", t.dim);
}

fn exit(parent: &mut ChildSpawner, t: &Theme) {
    panel(parent, t, 430.0, 180.0, 740.0, 420.0);
    text_at(parent, t, 520.0, 230.0, 24.0, "END SESSION", t.accent);
    text_at(parent, t, 520.0, 304.0, FONT_SIZE, "Return to the title screen?", t.text);
    text_at(parent, t, 520.0, 372.0, SMALL_FONT, "ENTER → TITLE", t.success);
    text_at(parent, t, 520.0, 410.0, SMALL_FONT, "ESC   → MAIN MENU", t.dim);
    text_at(parent, t, 520.0, 476.0, TINY_FONT, "This prototype does not terminate the application.", t.dim);
}

fn panel(parent: &mut ChildSpawner, t: &Theme, x: f32, y: f32, width: f32, height: f32) {
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            width: Val::Px(width),
            height: Val::Px(height),
            border: UiRect::all(Val::Px(1.0)),
            overflow: Overflow::clip(),
            ..default()
        },
        BorderColor::all(t.border),
        BackgroundColor(t.panel),
    ));
}

fn line(parent: &mut ChildSpawner, t: &Theme, x: f32, y: f32, width: f32, color: Color) {
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            width: Val::Px(width),
            height: Val::Px(1.0),
            ..default()
        },
        BackgroundColor(color),
    ));
}

fn selected_line(
    parent: &mut ChildSpawner,
    t: &Theme,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    selected: bool,
    hotkey: &str,
    label: &str,
) {
    if selected {
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(x),
                top: Val::Px(y),
                width: Val::Px(width),
                height: Val::Px(height),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderColor::all(t.accent),
            BackgroundColor(Color::srgba(1.0, 0.74, 0.16, 0.08)),
        ));
    }
    let pointer = if selected { "▶" } else { " " };
    text_at(parent, t, x + 12.0, y + 9.0, FONT_SIZE, &format!("{} [{}] {}", pointer, hotkey, label), if selected { t.accent } else { t.text });
}

fn text_at(parent: &mut ChildSpawner, _t: &Theme, x: f32, y: f32, size: f32, value: &str, color: Color) {
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            ..default()
        },
        Text::new(value),
        TextFont {
            font: FontSource::Monospace,
            font_size: FontSize::Px(size),
            font_smoothing: FontSmoothing::None,
            ..default()
        },
        TextColor(color),
        TextLayout::no_wrap(),
    ));
}

fn boxed_text(parent: &mut ChildSpawner, t: &Theme, x: f32, y: f32, width: f32, height: f32) {
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            width: Val::Px(width),
            height: Val::Px(height),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BorderColor::all(t.border),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.08)),
    ));
}

fn bar(parent: &mut ChildSpawner, t: &Theme, x: f32, y: f32, label: &str, current: u32, max: u32, color: Color) {
    let width = 270.0;
    let segments = 18usize;
    let filled = if max == 0 { 0 } else { ((current as f32 / max as f32) * segments as f32).round() as usize }.min(segments);
    let mut glyphs = String::new();
    for i in 0..segments {
        glyphs.push(if i < filled { '█' } else { '░' });
    }
    text_at(parent, t, x, y, SMALL_FONT, &format!("{label}  {glyphs}  {current}/{max}"), color);
    line(parent, t, x, y + 23.0, width, t.border);
}
