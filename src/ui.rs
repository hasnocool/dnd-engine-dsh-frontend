// src/ui.rs
use bevy::prelude::*;

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
    pub fn reset(&mut self, scene: Scene) {
        self.selected = 0;
        if scene.menu_items().is_empty() {
            self.selected = 0;
        }
    }

    pub fn move_up(&mut self, scene: Scene) -> bool {
        let count = scene.menu_items().len();
        if count == 0 {
            return false;
        }
        self.selected = if self.selected == 0 { count - 1 } else { self.selected - 1 };
        true
    }

    pub fn move_down(&mut self, scene: Scene) -> bool {
        let count = scene.menu_items().len();
        if count == 0 {
            return false;
        }
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
            Scene::GameBoard => Some(Scene::Combat),
            Scene::StatsInventory | Scene::Map | Scene::Combat | Scene::Dialog => {
                Some(Scene::GameBoard)
            }
            Scene::Settings | Scene::Credits => Some(Scene::MainMenu),
            Scene::Exit => Some(Scene::Title),
            Scene::MainMenu => None,
        }
    }

    pub fn number_target(&self, scene: Scene, number: usize) -> Option<Scene> {
        if number == 0 {
            return scene.menu_items().iter().find(|item| item.hotkey == "0").map(|item| item.target);
        }
        scene.menu_items().iter()
            .find(|item| item.hotkey == number.to_string())
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
            Scene::StatsInventory | Scene::Inventory | Scene::Map | Scene::Combat | Scene::Dialog => {
                Some(Scene::GameBoard)
            }
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
    let root = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0)),
            row_gap: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(theme.background),
        UiRoot,
    )).id();

    commands.entity(root).with_children(|page| {
        header(page, theme, scene);

        match scene {
            Scene::Title => title_scene(page, theme),
            Scene::MainMenu => main_menu(page, theme, nav.selected),
            Scene::CharacterCreation => character_creation(page, theme, nav.selected),
            Scene::GameBoard => game_board(page, theme, nav.selected),
            Scene::StatsInventory => stats_inventory(page, theme),
            Scene::Map => map_scene(page, theme),
            Scene::Inventory => inventory_scene(page, theme),
            Scene::Combat => combat_scene(page, theme, nav.selected),
            Scene::Dialog => dialog_scene(page, theme, nav.selected),
            Scene::Settings => settings_scene(page, theme, nav.selected),
            Scene::Credits => credits_scene(page, theme),
            Scene::Exit => exit_scene(page, theme),
        }

        footer(page, theme, scene);
    });
}

fn header(parent: &mut ChildSpawner, theme: &Theme, scene: Scene) {
    let label = format!("╔{}╗", "═".repeat(108));
    text(parent, &label, 17.0, theme.accent);
    text(parent, &format!("║{:^108}║", scene.title()), 26.0, theme.text);
    text(parent, &format!("╚{}╝", "═".repeat(108)), 17.0, theme.accent);
}

fn footer(parent: &mut ChildSpawner, theme: &Theme, scene: Scene) {
    let hotkeys = match scene {
        Scene::GameBoard => "WASD / ↑↓←→ Move Nav   ENTER Select   I Inventory   M Map   C Combat   D Dialogue   ESC Back",
        Scene::MainMenu | Scene::CharacterCreation | Scene::Settings | Scene::Combat | Scene::Dialog => {
            "↑↓ / W S Navigate   ENTER Select   0-9 Quick Select   ESC Back   TAB Next"
        }
        _ => "ENTER Select   ESC Back   I Inventory   M Map   C Combat   D Dialogue",
    };
    text(parent, hotkeys, 14.0, theme.dim);
    text(parent, "D&D RPG ENGINE  ·  FRONTEND WIREFRAME  ·  CP437 + UNICODE TEXTMODE  ·  NAVIGATION ONLY", 12.0, theme.dim);
}

fn title_scene(parent: &mut ChildSpawner, theme: &Theme) {
    row(parent, 1.0, |layout| {
        panel(layout, theme, 30.0, |p| {
            text(p, "        ╔══════════════════════════╗", 18.0, theme.primary);
            text(p, "        ║       D&D RPG ENGINE      ║", 22.0, theme.text);
            text(p, "        ║        ╔╗  ╔╗  ╔╗        ║", 22.0, theme.accent);
            text(p, "        ╚══════════════════════════╝", 18.0, theme.primary);
            spacer(p, 12.0);
            text(p, "    ", 8.0, theme.dim);
            text(p, "    \"Not all those who wander are lost…\"", 19.0, theme.text);
            spacer(p, 22.0);
            selected_line(p, theme, true, "▶", "New Game");
            text(p, "          Continue", 21.0, theme.text);
            text(p, "          Settings", 21.0, theme.text);
            text(p, "          Quit", 21.0, theme.text);
            spacer(p, 18.0);
            text(p, "v0.1.0  ·  16:9 TERMINAL FRONTEND", 12.0, theme.dim);
        });
        panel(layout, theme, 70.0, |p| {
            text(p, "                         ▲", 18.0, theme.primary);
            text(p, "                        ╱ ╲", 18.0, theme.primary);
            text(p, "                 ╱╲    ╱███╲     ╱╲", 18.0, theme.primary);
            text(p, "                ╱██╲  ╱█████╲   ╱██╲", 18.0, theme.primary);
            text(p, "               ╱████╲ ║█████║  ╱████╲", 18.0, theme.primary);
            text(p, "              ╱██████╲║█████║ ╱██████╲", 18.0, theme.primary);
            text(p, "                 ║        ║        ║", 18.0, theme.dim);
            text(p, "              ═══╩════════╩════════╩═══", 18.0, theme.dim);
            spacer(p, 12.0);
            text(p, "        ☼  A terminal-born fantasy adventure  ☼", 16.0, theme.accent);
            text(p, "        ⚔  Explore · Fight · Discover · Survive  ⚔", 16.0, theme.text);
        });
    });
}

fn main_menu(parent: &mut ChildSpawner, theme: &Theme, selected: usize) {
    row(parent, 1.0, |layout| {
        panel(layout, theme, 45.0, |p| {
            text(p, "                    MAIN MENU", 22.0, theme.text);
            spacer(p, 10.0);
            let items = [
                ("1", "New Adventure"),
                ("2", "Continue"),
                ("3", "Load Game"),
                ("4", "Settings"),
                ("5", "Credits"),
                ("6", "Quit"),
            ];
            for (i, (key, label)) in items.iter().enumerate() {
                selected_line(p, theme, selected == i, &format!("[{}]", key), label);
            }
        });
        panel(layout, theme, 55.0, |p| {
            text(p, "               ╲  DUNGEON REALMS  ╱", 18.0, theme.primary);
            text(p, "                    ╱╲", 18.0, theme.primary);
            text(p, "              ╱╲   ╱██╲   ╱╲", 18.0, theme.primary);
            text(p, "             ╱██╲ ╱████╲ ╱██╲", 18.0, theme.primary);
            text(p, "            ║████║║████║║████║", 18.0, theme.primary);
            spacer(p, 18.0);
            text(p, "                " , 10.0, theme.dim);
            text(p, "                ★  Adventure awaits…  ★", 17.0, theme.accent);
            spacer(p, 8.0);
            text(p, "                ─────────────────────", 15.0, theme.dim);
            text(p, "                [1] Create a character", 15.0, theme.text);
            text(p, "                [2] Resume your quest", 15.0, theme.text);
            text(p, "                [3] Load a saved journey", 15.0, theme.text);
        });
    });
}

fn character_creation(parent: &mut ChildSpawner, theme: &Theme, selected: usize) {
    row(parent, 1.0, |layout| {
        panel(layout, theme, 35.0, |p| {
            text(p, "Create Your Hero", 20.0, theme.text);
            spacer(p, 6.0);
            let tabs = ["Race", "Class", "Background", "Abilities", "Appearance", "Name"];
            for (i, label) in tabs.iter().enumerate() {
                selected_line(p, theme, selected == i, "▶", label);
            }
            spacer(p, 12.0);
            text(p, "[7] NEXT  →  ENTER", 14.0, theme.accent);
            text(p, "[ESC] BACK", 14.0, theme.dim);
        });
        panel(layout, theme, 43.0, |p| {
            text(p, "Select Race", 20.0, theme.text);
            text(p, "┌──────────────────────────────┐", 16.0, theme.border);
            for race in ["Human", "Elf", "Dwarf", "Halfling", "Dragonborn", "Tiefling", "Half-Elf", "Half-Orc"] {
                text(p, &format!("│ {:<28} │", race), 16.0, theme.text);
            }
            text(p, "└──────────────────────────────┘", 16.0, theme.border);
            spacer(p, 8.0);
            text(p, "Human", 18.0, theme.accent);
            text(p, "Versatile and ambitious. No special", 14.0, theme.text);
            text(p, "bonuses, but excels on any path.", 14.0, theme.text);
            spacer(p, 5.0);
            text(p, "+1 STR   +1 CON", 16.0, theme.success);
        });
        panel(layout, theme, 22.0, |p| {
            text(p, "     ╱╲", 16.0, theme.primary);
            text(p, "    ╱██╲", 16.0, theme.primary);
            text(p, "   ║████║", 16.0, theme.primary);
            text(p, "    ╲██╱", 16.0, theme.primary);
            text(p, "     ╲╱", 16.0, theme.primary);
            spacer(p, 8.0);
            text(p, "STR 14", 15.0, theme.text);
            text(p, "DEX 12", 15.0, theme.text);
            text(p, "CON 13", 15.0, theme.text);
            text(p, "INT 10", 15.0, theme.text);
            text(p, "WIS 11", 15.0, theme.text);
            text(p, "CHA 08", 15.0, theme.text);
        });
    });
}

fn game_board(parent: &mut ChildSpawner, theme: &Theme, selected: usize) {
    row(parent, 1.0, |layout| {
        panel(layout, theme, 69.0, |p| {
            text(p, "OLD MINES — LEVEL 2", 17.0, theme.accent);
            for line in dungeon_map() {
                text(p, line, 13.0, theme.text);
            }
            spacer(p, 6.0);
            text(p, "▶ You see a goblin (Lv 2)", 15.0, theme.warning);
            text(p, "▶ Press ENTER to act…", 14.0, theme.dim);
        });
        panel(layout, theme, 31.0, |p| {
            text(p, "ARTHAS  Lv 3  Fighter", 18.0, theme.text);
            text(p, "HP  ██████████████░░  28/28", 14.0, theme.success);
            text(p, "MP  ██████████░░░░░░  10/10", 14.0, theme.primary);
            text(p, "XP  ███████░░░░░░░░░  34/100", 14.0, theme.accent);
            spacer(p, 6.0);
            text(p, "Location: Old Mines — L2", 13.0, theme.text);
            text(p, "────────────────────────────", 13.0, theme.dim);
            selected_line(p, theme, selected == 0, "[I]", "Inventory / Stats");
            selected_line(p, theme, selected == 1, "[M]", "World Map");
            selected_line(p, theme, selected == 2, "[C]", "Combat");
            selected_line(p, theme, selected == 3, "[D]", "Dialogue");
        });
    });
}

fn stats_inventory(parent: &mut ChildSpawner, theme: &Theme) {
    row(parent, 1.0, |layout| {
        panel(layout, theme, 47.0, |p| {
            text(p, "CHARACTER STATS", 18.0, theme.text);
            text(p, "────────────────────────", 14.0, theme.dim);
            for row in ["STR  14  (+2)", "DEX  12  (+1)", "CON  13  (+1)", "INT  10  (+0)", "WIS  11  (+0)", "CHA   8  (-1)"] {
                text(p, row, 16.0, theme.text);
            }
            spacer(p, 8.0);
            text(p, "Saving Throws", 15.0, theme.accent);
            text(p, "Fort +4   Ref +2   Will +1", 14.0, theme.text);
            spacer(p, 4.0);
            text(p, "Skills", 15.0, theme.accent);
            text(p, "Athletics +4   Stealth +3", 14.0, theme.text);
            text(p, "Perception +1   Arcana +0", 14.0, theme.text);
        });
        panel(layout, theme, 53.0, |p| {
            text(p, "INVENTORY", 18.0, theme.text);
            text(p, "────────────────────────────────", 14.0, theme.dim);
            text(p, "⚔  Longsword                 (e)", 16.0, theme.text);
            text(p, "◈  Wooden Shield              (e)", 16.0, theme.text);
            text(p, "♥  Healing Potion x3", 16.0, theme.success);
            text(p, "◇  Rations x5", 16.0, theme.text);
            text(p, "†  Torch x4", 16.0, theme.warning);
            text(p, "★  Gold 125", 16.0, theme.accent);
            spacer(p, 8.0);
            text(p, "Weight: 12 / 50", 14.0, theme.dim);
            text(p, "[I] Inventory  [M] Map  [ESC] Back", 13.0, theme.accent);
        });
    });
}

fn map_scene(parent: &mut ChildSpawner, theme: &Theme) {
    row(parent, 1.0, |layout| {
        panel(layout, theme, 78.0, |p| {
            text(p, "WORLD MAP", 18.0, theme.text);
            for line in world_map() {
                text(p, line, 12.0, theme.text);
            }
        });
        panel(layout, theme, 22.0, |p| {
            text(p, "LEGEND", 17.0, theme.text);
            text(p, "@  Player", 13.0, theme.accent);
            text(p, "⌂  Town", 13.0, theme.text);
            text(p, "☠  Dungeon", 13.0, theme.danger);
            text(p, "▲  Forest", 13.0, theme.success);
            text(p, "╱╲ Mountains", 13.0, theme.text);
            text(p, "≈≈ Water", 13.0, theme.primary);
            text(p, "✦  Point of Interest", 13.0, theme.warning);
        });
    });
}

fn inventory_scene(parent: &mut ChildSpawner, theme: &Theme) {
    row(parent, 1.0, |layout| {
        panel(layout, theme, 61.0, |p| {
            text(p, "INVENTORY   │   EQUIPMENT   │   KEY ITEMS   │   LOOT", 16.0, theme.accent);
            text(p, "────────────────────────────────────────────────────────", 13.0, theme.dim);
            for item in [
                "⚔ Longsword          1    5.0 lb    50 gp",
                "◈ Wooden Shield       1    6.0 lb    40 gp",
                "♥ Healing Potion      3    0.5 lb     25 gp",
                "◇ Rations             5    0.2 lb      5 gp",
                "† Torch               4    0.1 lb      2 gp",
                "★ Gold              125       —      125 gp",
            ] {
                text(p, item, 15.0, theme.text);
            }
            spacer(p, 8.0);
            text(p, "Total Weight: 12 / 50", 14.0, theme.dim);
        });
        panel(layout, theme, 39.0, |p| {
            text(p, "⚔  LONGSWORD", 18.0, theme.accent);
            text(p, "──────────────────", 13.0, theme.dim);
            text(p, "A well-crafted longsword.", 14.0, theme.text);
            text(p, "Balanced and reliable.", 14.0, theme.text);
            spacer(p, 6.0);
            text(p, "Damage: 1d8+2", 14.0, theme.danger);
            text(p, "Weight: 5.0", 14.0, theme.text);
            text(p, "Value: 50", 14.0, theme.accent);
            spacer(p, 9.0);
            text(p, "[E] Equip   [D] Drop   [ENTER] Details", 12.0, theme.dim);
        });
    });
}

fn combat_scene(parent: &mut ChildSpawner, theme: &Theme, selected: usize) {
    row(parent, 1.0, |layout| {
        panel(layout, theme, 28.0, |p| {
            text(p, "          GOBLIN", 18.0, theme.success);
            text(p, "             ╲▲╱", 17.0, theme.success);
            text(p, "            ╔███╗", 17.0, theme.success);
            text(p, "             ║█║", 17.0, theme.success);
            text(p, "            ╱ ║ ╲", 17.0, theme.success);
            text(p, "HP ██████████ 12/12", 13.0, theme.danger);
        });
        panel(layout, theme, 47.0, |p| {
            text(p, "⚔ COMBAT / BATTLE ⚔", 21.0, theme.accent);
            text(p, "Goblin  Lv 2", 16.0, theme.text);
            text(p, "HP ██████████████ 12/12", 14.0, theme.danger);
            spacer(p, 8.0);
            for (i, (key, label)) in [("1", "ATTACK"), ("2", "DEFEND"), ("3", "SKILLS"), ("4", "ITEMS"), ("5", "FLEE")].iter().enumerate() {
                selected_line(p, theme, selected == i, &format!("[{}]", key), label);
            }
        });
        panel(layout, theme, 25.0, |p| {
            text(p, "ARTHAS", 18.0, theme.text);
            text(p, "Lv 3 Fighter", 14.0, theme.dim);
            text(p, "HP ██████████████", 13.0, theme.success);
            text(p, "28 / 28", 13.0, theme.text);
            spacer(p, 12.0);
            text(p, "⚔  Battle state is static", 12.0, theme.dim);
            text(p, "No rules are executed.", 12.0, theme.dim);
        });
    });
    panel_full(parent, theme, |p| {
        text(p, "▶ You strike the Goblin for 6 damage!", 13.0, theme.text);
        text(p, "▶ Goblin attacks for 3 damage!", 13.0, theme.danger);
        text(p, "▶ Your turn.", 13.0, theme.accent);
    });
}

fn dialog_scene(parent: &mut ChildSpawner, theme: &Theme, selected: usize) {
    panel_full(parent, theme, |p| {
        row(p, 1.0, |layout| {
            panel(layout, theme, 25.0, |q| {
                text(q, "       ☼", 24.0, theme.primary);
                text(q, "      ╱╲", 20.0, theme.primary);
                text(q, "     ╱██╲", 20.0, theme.primary);
                text(q, "    ╱████╲", 20.0, theme.primary);
                text(q, "     ╲██╱", 20.0, theme.primary);
            });
            panel(layout, theme, 75.0, |q| {
                text(q, "ELDER SEER", 18.0, theme.accent);
                spacer(q, 5.0);
                text(q, "\"The path you seek is not easy, young one.", 16.0, theme.text);
                text(q, " To the east lies a dungeon filled with ancient", 16.0, theme.text);
                text(q, " dangers. Are you prepared?\"", 16.0, theme.text);
                spacer(q, 10.0);
                for (i, label) in ["I am ready.", "Tell me more.", "Leave."] .iter().enumerate() {
                    selected_line(q, theme, selected == i, &format!("{}.", i + 1), label);
                }
            });
        });
    });
}

fn settings_scene(parent: &mut ChildSpawner, theme: &Theme, selected: usize) {
    panel_full(parent, theme, |p| {
        text(p, "SETTINGS", 20.0, theme.text);
        text(p, "────────────────────────────────────────", 14.0, theme.dim);
        selected_line(p, theme, selected == 0, "[1]", "Audio");
        selected_line(p, theme, selected == 1, "[2]", "Display");
        selected_line(p, theme, selected == 2, "[3]", "Controls");
        text(p, "", 7.0, theme.dim);
        selected_line(p, theme, selected == 3, "[0]", "Back");
        spacer(p, 14.0);
        text(p, "Audio     ████████████  80%", 15.0, theme.text);
        text(p, "Display   1600×900  ·  16:9  ·  Terminal Mode", 15.0, theme.text);
        text(p, "Controls  W A S D  ·  Arrows  ·  Enter  ·  Esc", 15.0, theme.text);
    });
}

fn credits_scene(parent: &mut ChildSpawner, theme: &Theme) {
    panel_full(parent, theme, |p| {
        text(p, "D&D RPG ENGINE FRONTEND", 24.0, theme.accent);
        spacer(p, 8.0);
        text(p, "Rust  ·  Bevy  ·  Winit", 18.0, theme.text);
        text(p, "CP437-inspired Unicode / ANSI textmode presentation", 15.0, theme.text);
        spacer(p, 14.0);
        text(p, "Wireframe-only prototype. Navigation is intentionally", 15.0, theme.text);
        text(p, "the only implemented behavior. No combat, persistence,", 15.0, theme.text);
        text(p, "networking, or game rules are executed.", 15.0, theme.text);
        spacer(p, 12.0);
        text(p, "✓ Scene navigation", 14.0, theme.success);
        text(p, "✓ Keyboard hotkeys", 14.0, theme.success);
        text(p, "✓ 16:9 terminal UI system", 14.0, theme.success);
        text(p, "✓ Unicode / textmode glyph system", 14.0, theme.success);
    });
}

fn exit_scene(parent: &mut ChildSpawner, theme: &Theme) {
    panel_full(parent, theme, |p| {
        text(p, "☠  END SESSION  ☠", 24.0, theme.danger);
        spacer(p, 8.0);
        text(p, "The Quit scene is a navigation wireframe only.", 16.0, theme.text);
        spacer(p, 8.0);
        text(p, "ENTER  →  Return to Title", 15.0, theme.accent);
        text(p, "ESC    →  Return to Main Menu", 15.0, theme.dim);
    });
}

fn dungeon_map() -> Vec<&'static str> {
    vec![
        "┌──────────────────────────────────────────────────────────────────┐",
        "│ ████████████        ░░░░░░░░░░        ████████████████████████ │",
        "│ █          █        ░  ◇       ░        █                     █ │",
        "│ █   @      █────────░───────────░────────█      ☠             █ │",
        "│ █          █        ░     g     ░        █                     █ │",
        "│ █████  █████        ░░░░░░░░░░░░        ████████████  ████████ │",
        "│       ╲        ┌────────────────┐                 ╲           │",
        "│        ╲───────│  ═══════════   │──────────────────╲         │",
        "│                │       $        │                    ╲        │",
        "│      ✦         │   ░░░░░░░░     │                     ╲       │",
        "│                └──────┬─────────┘                      ╲      │",
        "│                       │                                  !    │",
        "│                  ╔════╧════╗                                   │",
        "│                  ║  CHEST  ║          ≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈    │",
        "│                  ╚═════════╝          ≈  UNDERGROUND LAKE ≈    │",
        "└──────────────────────────────────────────────────────────────────┘",
    ]
}

fn world_map() -> Vec<&'static str> {
    vec![
        "┌───────────────────────────────────────────────────────────────────────┐",
        "│ ▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲      ╱╲          ▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲ │",
        "│ ▲▲▲▲▲▲   ▲▲▲▲▲▲▲▲▲▲     ╱██╲         ▲▲▲▲▲▲▲▲▲▲▲▲▲▲ │",
        "│ ▲▲▲      ▲▲▲▲▲▲▲▲▲▲    ╱████╲        ▲▲▲▲▲▲▲▲▲▲▲▲ │",
        "│ ▲  ⌂ TOWN     ────────╱██████╲───────────────▲▲▲▲ │",
        "│ ▲              ╲      ╲██████╱       ╲          ▲▲▲ │",
        "│ ▲               ╲       ╲██╱          ╲            │",
        "│ ▲▲▲▲▲▲▲▲        ╲        ║            ╲           │",
        "│ ▲▲▲▲▲▲▲▲▲▲       ╲═══════╬═════════════╲        │",
        "│ ▲▲▲▲▲▲▲▲▲▲▲              @                ☠ DUNGEON│",
        "│       ≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈  │",
        "│       ≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈  │",
        "│              ✦ ANCIENT ROAD        ✦            │",
        "└───────────────────────────────────────────────────────────────────────┘",
    ]
}

fn row(parent: &mut ChildSpawner, width: f32, build: impl FnOnce(&mut ChildSpawner)) {
    let id = parent.spawn(Node {
        width: Val::Percent(100.0 * width),
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(12.0),
        flex_grow: 1.0,
        min_height: Val::Px(0.0),
        ..default()
    }).id();
    parent.entity_mut(id).with_children(build);
}

fn panel(parent: &mut ChildSpawner, theme: &Theme, width_percent: f32, build: impl FnOnce(&mut ChildSpawner)) {
    let id = parent.spawn((
        Node {
            width: Val::Percent(width_percent),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(12.0)),
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(1.0)),
            overflow: Overflow::clip(),
            ..default()
        },
        BorderColor::all(theme.border),
        BackgroundColor(theme.panel),
    )).id();
    parent.entity_mut(id).with_children(build);
}

fn panel_full(parent: &mut ChildSpawner, theme: &Theme, build: impl FnOnce(&mut ChildSpawner)) {
    let id = parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            padding: UiRect::all(Val::Px(14.0)),
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(1.0)),
            overflow: Overflow::clip(),
            ..default()
        },
        BorderColor::all(theme.border),
        BackgroundColor(theme.panel),
    )).id();
    parent.entity_mut(id).with_children(build);
}

fn selected_line(parent: &mut ChildSpawner, theme: &Theme, selected: bool, marker: &str, label: &str) {
    let prefix = if selected { "▶" } else { " " };
    let line = format!("{} {:<4} {}", prefix, marker, label);
    text(parent, &line, 17.0, if selected { theme.accent } else { theme.text });
}

fn spacer(parent: &mut ChildSpawner, height: f32) {
    parent.spawn(Node {
        height: Val::Px(height),
        width: Val::Percent(100.0),
        flex_shrink: 0.0,
        ..default()
    });
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
