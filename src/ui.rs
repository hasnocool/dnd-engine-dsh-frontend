// src/ui.rs
use bevy::prelude::*;
use std::fmt::Write;

use crate::theme::Theme;

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
        let count = scene.menu_items().len();
        if count == 0 { return false; }
        self.selected = if self.selected == 0 { count - 1 } else { self.selected - 1 };
        true
    }

    pub fn move_down(&mut self, scene: Scene) -> bool {
        let count = scene.menu_items().len();
        if count == 0 { return false; }
        self.selected = (self.selected + 1) % count;
        true
    }

    pub fn selected_target(&self, scene: Scene) -> Option<Scene> {
        if let Some(item) = scene.menu_items().get(self.selected) {
            return Some(item.target);
        }
        match scene {
            Scene::Title => Some(Scene::MainMenu),
            Scene::CharacterCreation => Some(Scene::GameBoard),
            Scene::Combat | Scene::Dialog => Some(Scene::GameBoard),
            Scene::StatsInventory | Scene::Map => Some(Scene::GameBoard),
            Scene::Credits => Some(Scene::MainMenu),
            Scene::Exit => Some(Scene::Title),
            Scene::GameBoard => Some(Scene::Combat),
            Scene::Settings => Some(Scene::MainMenu),
            Scene::MainMenu => None,
        }
    }

    pub fn number_target(&self, scene: Scene, number: usize) -> Option<Scene> {
        if number == 0 {
            return (scene == Scene::Settings).then_some(Scene::MainMenu);
        }
        scene.menu_items().get(number.saturating_sub(1)).map(|item| item.target)
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
            Scene::GameBoard
            | Scene::StatsInventory
            | Scene::Map
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
        const SETTINGS: &[NavItem] = &[
            NavItem { label: "Audio", target: Scene::Settings, hotkey: "1" },
            NavItem { label: "Display", target: Scene::Settings, hotkey: "2" },
            NavItem { label: "Controls", target: Scene::Settings, hotkey: "3" },
            NavItem { label: "Back", target: Scene::MainMenu, hotkey: "0" },
        ];
        match self {
            Scene::MainMenu => MAIN,
            Scene::Settings => SETTINGS,
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
    let root = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(18.0)),
            row_gap: Val::Px(10.0),
            ..default()
        },
        BackgroundColor(theme.background),
        UiRoot,
    )).id();

    commands.entity(root).with_children(|parent| {
        let top = format!("╔{}╗", "═".repeat(88));
        let mid = format!("║{:^88}║", scene.title());
        let bottom = format!("╚{}╝", "═".repeat(88));
        text(parent, &top, 18.0, theme.accent);
        text(parent, &mid, 26.0, theme.text);
        text(parent, &bottom, 18.0, theme.accent);

        match scene {
            Scene::Title => title_scene(parent, theme),
            Scene::MainMenu => menu_scene(parent, theme, nav.selected, scene.menu_items()),
            Scene::CharacterCreation => character_scene(parent, theme),
            Scene::GameBoard => game_board(parent, theme),
            Scene::StatsInventory => stats_inventory(parent, theme),
            Scene::Map => world_map(parent, theme),
            Scene::Combat => combat(parent, theme),
            Scene::Dialog => dialog(parent, theme),
            Scene::Settings => menu_scene(parent, theme, nav.selected, scene.menu_items()),
            Scene::Credits => credits(parent, theme),
            Scene::Exit => exit_scene(parent, theme),
        }

        text(parent, "", 6.0, theme.dim);
        text(parent, "ESC Back   ENTER Select   ↑↓ / W S Navigate   1-6 Quick Select", 16.0, theme.dim);
        text(parent, "D&D RPG ENGINE FRONTEND  ·  Navigation-only prototype  ·  No gameplay functions", 13.0, theme.dim);
    });
}

fn title_scene(parent: &mut ChildSpawner, theme: &Theme) {
    text(parent, "                 ██████   ██████  ███████", 26.0, theme.primary);
    text(parent, "                D&D      RPG      ENGINE", 26.0, theme.text);
    text(parent, "", 10.0, theme.dim);
    text(parent, "         \"Not all those who wander are lost…\"", 20.0, theme.text);
    text(parent, "", 8.0, theme.dim);
    framed(parent, theme, &["                         ▶  NEW GAME", "                            ENTER", "                         v0.1.0  ·  UI WIREFRAME"]);
    text(parent, "Press ENTER to enter the Main Menu", 17.0, theme.accent);
}

fn menu_scene(parent: &mut ChildSpawner, theme: &Theme, selected: usize, items: &[NavItem]) {
    for (index, item) in items.iter().enumerate() {
        let marker = if index == selected { "▶" } else { " " };
        let line = format!("  {marker} [{}]  {}", item.hotkey, item.label);
        text(parent, &line, 22.0, if index == selected { theme.accent } else { theme.text });
    }
}

fn character_scene(parent: &mut ChildSpawner, theme: &Theme) {
    framed(parent, theme, &[
        "CREATE YOUR HERO                         SELECT RACE",
        "▶ Race                                   │ Human      │",
        "  Class                                  │ Elf        │",
        "  Background                             │ Dwarf      │",
        "  Abilities                              │ Halfling   │",
        "  Appearance                             │ Tiefling   │",
        "  Name                                   │ Half-Elf   │",
        "                                         │ Half-Orc   │",
        "",
        "Human — versatile and ambitious.          +1 STR  +1 CON",
        "",
        "                         [ < BACK ]      [ NEXT > ]",
        "",
        "ENTER advances to the Game Board. This screen contains no character logic.",
    ]);
}

fn game_board(parent: &mut ChildSpawner, theme: &Theme) {
    framed(parent, theme, &[
        "┌─────────────────────────────────┐  ┌─────────────────────────────┐",
        "│ ███ DUNGEON PREVIEW ███████████ │  │ ARTHAS  Lv 3  Fighter       │",
        "│                                 │  │ HP  ████████████░░ 28/28    │",
        "│ #######       $       #######   │  │ MP  ██████████░░░░ 10/10    │",
        "│ #.....#   g   #.....#     !     │  │ XP  █████████░░░░  34/100   │",
        "│ #..@..#####...#.....#           │  │ Location: Old Mines — L2     │",
        "│ #.......     #..g..#           │  │                              │",
        "│ #######       #######           │  │                              │",
        "└─────────────────────────────────┘  └─────────────────────────────┘",
        "",
        "▶ You see a goblin (Lv 2)",
        "▶ [C] Combat     [D] Dialogue     [I] Inventory/Stats     [M] World Map",
    ]);
}

fn stats_inventory(parent: &mut ChildSpawner, theme: &Theme) {
    framed(parent, theme, &[
        "┌──────────────────────────┐  ┌───────────────────────────────┐",
        "│ CHARACTER STATS          │  │ INVENTORY                     │",
        "│ STR  14  (+2)            │  │ ⚔ Longsword              (e) │",
        "│ DEX  12  (+1)            │  │ ◈ Wooden Shield            (e) │",
        "│ CON  13  (+1)            │  │ ♥ Healing Potion x3           │",
        "│ INT  10  (+0)            │  │ ◇ Rations x5                  │",
        "│ WIS  11  (+0)            │  │ † Torch x4                    │",
        "│ CHA   8  (-1)            │  │ ★ Gold 125                    │",
        "│                          │  │ Weight: 12 / 50               │",
        "└──────────────────────────┘  └───────────────────────────────┘",
        "",
        "[M] Map    [ESC] Back",
    ]);
}

fn world_map(parent: &mut ChildSpawner, theme: &Theme) {
    framed(parent, theme, &[
        "MAP / WORLD VIEW",
        "",
        "▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲",
        "▲▲▲▒▒▒▒▒▒▒▒▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲",
        "▲▲▲▒   TOWN   ▒▲▲▲▲▲▲  ╱╲  ▲▲▲▲▲",
        "▲▲▲▒      @   ▒▲▲▲▲▲▲ ╱██╲ ▲▲▲▲▲",
        "▲▲▲▒▒▒▒▒▒▒▒▒▒▒▒▲▲▲▲  ╱██╲  ▲▲▲▲",
        "▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲╲██╱▲▲▲▲▲",
        "",
        "Legend: @ Player   T Town   D Dungeon   ▲ Forest   ╱╲ Mountains",
        "[I] Stats/Inventory   [ESC] Back",
    ]);
}

fn combat(parent: &mut ChildSpawner, theme: &Theme) {
    framed(parent, theme, &[
        "                         ⚔  COMBAT  ⚔",
        "",
        "          GOBLIN Lv 2                ARTHAS Lv 3",
        "          HP ██████████ 12/12       HP ██████████ 28/28",
        "",
        "                 ▶ ATTACK",
        "                   DEFEND",
        "                   SKILLS",
        "                   ITEMS",
        "                   FLEE",
        "",
        "> Selecting an action returns to the Game Board.",
        "No combat rules are implemented in this frontend.",
        "ENTER Select Mocked Action    ESC Back",
    ]);
}

fn dialog(parent: &mut ChildSpawner, theme: &Theme) {
    framed(parent, theme, &[
        "Elder Seer",
        "",
        "\"The path you seek is not easy, young one. To the east lies",
        " a dungeon filled with ancient dangers. Are you prepared?\"",
        "",
        "▶ 1. I am ready.",
        "  2. Tell me more.",
        "  3. Leave.",
        "",
        "ENTER Select Dialogue Option    ESC Back",
    ]);
}

fn credits(parent: &mut ChildSpawner, theme: &Theme) {
    framed(parent, theme, &[
        "D&D RPG ENGINE FRONTEND",
        "Rust + Bevy + Winit",
        "",
        "Wireframe-only presentation layer.",
        "Navigation is intentionally the only implemented behavior.",
        "No combat, persistence, networking, or game rules.",
        "No GitHub Actions workflows.",
    ]);
}

fn exit_scene(parent: &mut ChildSpawner, theme: &Theme) {
    framed(parent, theme, &[
        "END SESSION",
        "",
        "▶ RETURN TO TITLE",
        "",
        "ENTER  Return to title",
        "ESC    Return to main menu",
    ]);
}

fn framed(parent: &mut ChildSpawner, theme: &Theme, lines: &[&str]) {
    let mut body = String::new();
    let width = 78usize;
    let _ = writeln!(&mut body, "┌{}┐", "─".repeat(width));
    for line in lines {
        let clipped = if line.chars().count() > width { line.chars().take(width).collect::<String>() } else { (*line).to_string() };
        let _ = writeln!(&mut body, "│{:<width$}│", clipped, width = width);
    }
    let _ = writeln!(&mut body, "└{}┘", "─".repeat(width));
    text(parent, &body, 18.0, theme.text);
}

fn text(parent: &mut ChildSpawner, value: &str, size: f32, color: Color) {
    parent.spawn((
        Text::new(value),
        TextFont {
            font: FontSource::Monospace,
            font_size: FontSize::Px(size),
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Left),
    ));
}
