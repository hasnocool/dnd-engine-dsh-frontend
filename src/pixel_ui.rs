// src/pixel_ui.rs
//! Scene definitions for the fixed 100×28 CP437 cell renderer.
//! The full existing scene-authored grid implementation lives in this module.

use bevy::prelude::*;

use crate::batched_renderer::spawn_batched_grid;
use crate::cp437_grid::{Cp437Atlas, Grid, GRID_COLS, GRID_ROWS};
use crate::theme::Theme;

#[derive(Component)]
pub struct UiRoot;

#[derive(Resource)]
pub struct CurrentScene {
    pub scene: Scene,
}

impl Default for CurrentScene {
    fn default() -> Self { Self { scene: Scene::Title } }
}

#[derive(Resource, Default)]
pub struct NavigationState { pub selected: usize }

impl NavigationState {
    pub fn reset(&mut self, _scene: Scene) { self.selected = 0; }
    pub fn move_up(&mut self, scene: Scene) -> bool {
        let len = scene.menu_items().len();
        if len == 0 { return false; }
        self.selected = if self.selected == 0 { len - 1 } else { self.selected - 1 };
        true
    }
    pub fn move_down(&mut self, scene: Scene) -> bool {
        let len = scene.menu_items().len();
        if len == 0 { return false; }
        self.selected = (self.selected + 1) % len;
        true
    }
    pub fn selected_target(&self, scene: Scene) -> Option<Scene> {
        scene.menu_items().get(self.selected).map(|item| item.target).or_else(|| match scene {
            Scene::Title => Some(Scene::MainMenu),
            Scene::GameBoard | Scene::StatsInventory | Scene::Inventory | Scene::Map | Scene::Combat | Scene::Dialog => Some(Scene::GameBoard),
            Scene::CharacterCreation => Some(Scene::GameBoard),
            Scene::Settings | Scene::Credits => Some(Scene::MainMenu),
            Scene::Exit => Some(Scene::Title),
            Scene::MainMenu => None,
        })
    }
    pub fn number_target(&self, scene: Scene, number: usize) -> Option<Scene> {
        let key = number.to_string();
        scene.menu_items().iter().find(|item| item.hotkey == key).map(|item| item.target)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scene {
    Title, MainMenu, CharacterCreation, GameBoard, StatsInventory, Map,
    Inventory, Combat, Dialog, Settings, Credits, Exit,
}

#[derive(Clone, Copy)]
pub struct NavItem { pub label: &'static str, pub target: Scene, pub hotkey: &'static str }

impl Scene {
    pub fn title(self) -> &'static str {
        match self {
            Scene::Title => "D&D RPG ENGINE", Scene::MainMenu => "MAIN MENU",
            Scene::CharacterCreation => "CREATE YOUR HERO", Scene::GameBoard => "MAIN GAME BOARD",
            Scene::StatsInventory => "STATS & INVENTORY", Scene::Map => "MAP / WORLD VIEW",
            Scene::Inventory => "INVENTORY", Scene::Combat => "COMBAT / BATTLE",
            Scene::Dialog => "DIALOG / NPC INTERACTION", Scene::Settings => "SETTINGS",
            Scene::Credits => "CREDITS", Scene::Exit => "END SESSION",
        }
    }
    pub fn back(self) -> Option<Self> {
        match self {
            Scene::Title => None, Scene::MainMenu => Some(Scene::Title),
            Scene::CharacterCreation => Some(Scene::MainMenu), Scene::GameBoard => Some(Scene::MainMenu),
            Scene::StatsInventory | Scene::Map | Scene::Inventory | Scene::Combat | Scene::Dialog => Some(Scene::GameBoard),
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
            NavItem { label: "Race", target: Scene::CharacterCreation, hotkey: "1" }, NavItem { label: "Class", target: Scene::CharacterCreation, hotkey: "2" },
            NavItem { label: "Background", target: Scene::CharacterCreation, hotkey: "3" }, NavItem { label: "Abilities", target: Scene::CharacterCreation, hotkey: "4" },
            NavItem { label: "Appearance", target: Scene::CharacterCreation, hotkey: "5" }, NavItem { label: "Name", target: Scene::CharacterCreation, hotkey: "6" },
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
            Scene::MainMenu => MAIN, Scene::CharacterCreation => CREATION, Scene::GameBoard => BOARD,
            Scene::Settings => SETTINGS, Scene::Combat => COMBAT, Scene::Dialog => DIALOG, _ => &[],
        }
    }
}

pub fn build_grid(theme: &Theme, scene: Scene, selected: usize) -> Grid {
    let mut g = Grid::new(theme.background);
    frame(&mut g, theme, scene.title());
    match scene {
        Scene::Title => title(&mut g, theme), Scene::MainMenu => main_menu(&mut g, theme, selected),
        Scene::CharacterCreation => creation(&mut g, theme, selected), Scene::GameBoard => board(&mut g, theme, selected),
        Scene::StatsInventory => stats(&mut g, theme), Scene::Map => map(&mut g, theme),
        Scene::Inventory => inventory(&mut g, theme), Scene::Combat => combat(&mut g, theme, selected),
        Scene::Dialog => dialog(&mut g, theme, selected), Scene::Settings => settings(&mut g, theme, selected),
        Scene::Credits => credits(&mut g, theme), Scene::Exit => exit(&mut g, theme),
    }
    footer(&mut g, theme);
    g
}

fn frame(g: &mut Grid, t: &Theme, title: &str) { g.box_double(0, 0, GRID_COLS, 3, t.accent, t.background); g.centered(1, title, t.text); }
fn footer(g: &mut Grid, t: &Theme) { g.hline(0, 25, GRID_COLS, t.border); g.text(2, 26, "↑↓/W S NAV   ENTER/SPACE SELECT   0-9 QUICK   ESC BACK   I INV   M MAP   C COMBAT   D DIALOG", t.dim); g.text(2, 27, "100×28 CELLS · 8×16 IBM VGA GLYPHS @ 2× · 1600×896 DRAWABLE · FIXED CELL FRONTEND", t.dim); }
fn panel(g:&mut Grid,t:&Theme,x:usize,y:usize,w:usize,h:usize,title:&str){g.fill(x,y,w,h,t.panel);g.box_single(x,y,w,h,t.border,t.panel);if !title.is_empty(){g.text(x+2,y,&format!(" {title} "),t.accent);}}
fn title(g:&mut Grid,t:&Theme){panel(g,t,3,4,43,19,"D&D RPG ENGINE");panel(g,t,49,4,48,19,"ADVENTURE AWAITS");g.text(7,6,"╔════════════════╗",t.primary);g.text(7,7,"║    D&D RPG    ║",t.text);g.text(7,8,"║     ENGINE     ║",t.text);g.text(7,9,"╚════════════════╝",t.primary);g.text(7,12,"Not all those who wander are lost...",t.text);g.text(7,14,"A TERMINAL-STYLE FANTASY ADVENTURE",t.dim);g.selection(7,16,35,true,"1","NEW GAME",t);g.selection(7,17,35,false,"2","CONTINUE",t);g.selection(7,18,35,false,"3","SETTINGS",t);g.selection(7,19,35,false,"4","QUIT",t);g.text(63,7,"▲",t.primary);g.text(56,8,"╱╲   ╱███╲   ╱╲",t.primary);g.text(54,9,"╱██╲ ╱█████╲ ╱██╲",t.primary);g.text(52,10,"║████║║█████║║████║",t.primary);g.text(65,11,"═══╩═══",t.dim);g.centered_at(13,49,48,"★  EXPLORE · FIGHT · DISCOVER · SURVIVE  ★",t.accent);g.centered_at(16,49,48,"CP437 TEXTMODE / VGA STYLE",t.text);g.centered_at(18,49,48,"VERSION 0.1.0 · FRONTEND WIREFRAME",t.dim);}
fn main_menu(g:&mut Grid,t:&Theme,selected:usize){panel(g,t,4,4,40,19,"MAIN MENU");panel(g,t,46,4,50,19,"THE OLD KINGDOM");let items=["New Adventure","Continue","Load Game","Settings","Credits","Quit"];for(i,item)in items.iter().enumerate(){g.selection(7,7+i,34,selected==i,&(i+1).to_string(),item,t);}g.text(59,7,"╱╲",t.primary);g.text(55,8,"╱██╲    ╱╲",t.primary);g.text(53,9,"║██║ ╱██╲ ║",t.primary);g.text(51,10,"║████║████║║",t.primary);g.centered_at(14,46,50,"The road begins where the map ends.",t.text);}
fn creation(g:&mut Grid,t:&Theme,selected:usize){panel(g,t,2,4,28,19,"CREATE YOUR HERO");panel(g,t,31,4,45,19,"SELECT RACE");panel(g,t,77,4,21,19,"PREVIEW");for(i,item)in ["Race","Class","Background","Abilities","Appearance","Name"].iter().enumerate(){g.selection(4,7+i,23,selected==i,&(i+1).to_string(),item,t);}g.text(4,15,"[7] NEXT → ENTER",t.accent);for(i,race)in ["Human","Elf","Dwarf","Halfling","Dragonborn","Tiefling","Half-Elf","Half-Orc"].iter().enumerate(){g.text(35,6+i,&format!("│ {:<35} │",race),t.text);}g.text(35,15,"Selected: Human",t.accent);g.text(35,16,"+1 STR  +1 CON",t.success);g.text(35,18,"Versatile and ambitious.",t.text);g.text(84,7,"  ╱██╲",t.primary);g.text(83,8," ║████║",t.primary);g.text(84,9,"  ╲██╱",t.primary);for(i,s)in ["STR 14","DEX 12","CON 13","INT 10","WIS 11","CHA 08"].iter().enumerate(){g.text(82,12+i,s,t.text);}}
fn board(g:&mut Grid,t:&Theme,selected:usize){panel(g,t,1,4,72,19,"OLD MINES — LEVEL 2");panel(g,t,74,4,25,19,"ARTHAS · LV 3 · FIGHTER");let dungeon=["┌──────────────────────────────────────────────────────────────────┐","│..............     ..............         ..............           │","│..┌────────┐      ..┌────────┐            ..┌────────┐           │","│..│        │.......  │        │............│        │           │","│..│   @    │   g     │   ▓▓   │            │   ☠    │           │","│..│        │.........│        │............│        │           │","│..└────────┘          └────────┘            └────────┘           │","│             ░░░░░░░░░░░░░░░░                                  │","│                 ┌───────┐                                      │","│                 │   □   │       ▲ EXIT                        │","│                 └───────┘                                      │","└──────────────────────────────────────────────────────────────────┘"];for(i,line)in dungeon.iter().enumerate(){g.text(2,6+i,line,t.text);}g.text(5,20,"▶ You see a goblin (Lv 2)",t.warning);g.text(77,7,"HP ██████████████░░",t.success);g.text(77,8,"MP ██████████░░░░░░",t.primary);g.text(77,9,"XP ███████░░░░░░░░",t.accent);for(i,item)in ["Inventory / Stats","World Map","Combat","Dialogue"].iter().enumerate(){g.selection(77,12+i,19,selected==i,&(i+1).to_string(),item,t);}}
fn stats(g:&mut Grid,t:&Theme){panel(g,t,3,4,47,19,"CHARACTER STATS");panel(g,t,51,4,46,19,"INVENTORY");for(i,line)in ["STR 14 (+2)","DEX 12 (+1)","CON 13 (+1)","INT 10 (+0)","WIS 11 (+0)","CHA 08 (-1)","Saving Throws  Fort +4  Ref +2  Will +1","Skills Athletics +4  Stealth +3  Perception +1"].iter().enumerate(){g.text(6,7+i,line,t.text);}for(i,line)in ["⚔ Longsword                 (e)","◈ Wooden Shield              (e)","♥ Healing Potion x3","◇ Rations x5","† Torch x4","★ Gold 125"].iter().enumerate(){g.text(54,7+i,line,t.text);}g.text(54,16,"Weight: 12 / 50",t.dim);g.text(54,18,"I  Inventory    M  Map",t.accent);}
fn map(g:&mut Grid,t:&Theme){panel(g,t,2,4,75,19,"WORLD MAP");panel(g,t,79,4,19,19,"LEGEND");for(i,line)in ["╲      ╱╲         ╱╲       ╱╲"," ╲    ╱██╲  ▲    ╱██╲     ╱██╲","  ╲__╱████╲_____╱████╲___╱████╲","        ≈≈≈≈≈≈≈≈≈≈≈≈≈≈","   ⌂──────────@──────────✦","       │       │","       │   ☠   │","       │  ╱╲   │","       └─╱██╲──┘"].iter().enumerate(){g.text(7,7+i,line,t.text);}for(i,line)in ["@  Player","⌂  Town","☠  Dungeon","▲  Forest","╱╲ Mountains","≈  Water","✦  Interest"].iter().enumerate(){g.text(82,7+i,line,t.text);}}
fn inventory(g:&mut Grid,t:&Theme){panel(g,t,2,4,61,19,"INVENTORY · EQUIPMENT · KEY ITEMS · LOOT");panel(g,t,65,4,33,19,"ITEM DETAIL");for(i,line)in ["⚔ Longsword       1   5.0 lb   50 gp","◈ Wooden Shield    1   6.0 lb   40 gp","♥ Healing Potion   3   0.5 lb   25 gp","◇ Rations          5   0.2 lb    5 gp","† Torch             4   0.1 lb    2 gp","★ Gold            125          125 gp"].iter().enumerate(){g.text(5,8+i,line,t.text);}g.text(69,7,"⚔ LONGSWORD",t.accent);g.text(69,9,"A well-crafted longsword.",t.text);g.text(69,11,"Damage: 1d8+2",t.danger);g.text(69,12,"Weight: 5.0",t.text);g.text(69,13,"Value: 50",t.accent);}
fn combat(g:&mut Grid,t:&Theme,selected:usize){panel(g,t,2,4,32,19,"ENEMY");panel(g,t,35,4,31,19,"COMMANDS");panel(g,t,67,4,31,19,"PLAYER");g.text(13,8,"╲████╱",t.danger);g.text(11,9,"☠  GOBLIN  ☠",t.warning);g.text(12,11,"HP ████████░░",t.danger);for(i,item)in ["ATTACK","DEFEND","SKILLS","ITEMS","FLEE"].iter().enumerate(){g.selection(39,8+i,23,selected==i,&(i+1).to_string(),item,t);}g.text(71,7,"ARTHAS  Lv 3",t.text);g.text(71,9,"HP ██████████████░░",t.success);g.text(71,10,"MP ██████████░░░░░░",t.primary);g.text(71,12,"> Goblin raises its club.",t.dim);g.text(71,13,"> Your turn.",t.accent);}
fn dialog(g:&mut Grid,t:&Theme,selected:usize){panel(g,t,2,4,28,19,"ELDER SEER");panel(g,t,31,4,67,19,"DIALOGUE");g.text(10,8,"  ╱██╲",t.primary);g.text(9,9," ║████║",t.primary);g.text(9,10," ║████║",t.primary);g.text(10,11,"  ╲██╱",t.primary);g.text(35,7,"The path you seek is not easy, young one.",t.text);g.text(35,9,"To the east lies a dungeon filled with ancient",t.text);g.text(35,10,"dangers. Are you prepared?",t.text);for(i,item)in ["I am ready.","Tell me more.","Leave."].iter().enumerate(){g.selection(35,14+i,52,selected==i,&(i+1).to_string(),item,t);}}
fn settings(g:&mut Grid,t:&Theme,selected:usize){panel(g,t,21,4,58,19,"SETTINGS");for(i,item)in ["Audio","Display","Controls","Back"].iter().enumerate(){let key=if i==3{"0".to_string()}else{(i+1).to_string()};g.selection(27,8+i,46,selected==i,&key,item,t);}g.text(28,15,"PLACEHOLDER · NO SETTINGS ARE MUTATED",t.dim);}
fn credits(g:&mut Grid,t:&Theme){panel(g,t,18,4,64,19,"CREDITS");g.centered_at(8,18,64,"D&D RPG ENGINE",t.text);g.centered_at(10,18,64,"Rust · Bevy 0.19.1 · Winit 0.30.13",t.text);g.centered_at(12,18,64,"IBM VGA 8×16 CP437 cell renderer",t.accent);g.centered_at(14,18,64,"Frontend-only navigation prototype",t.dim);g.centered_at(17,18,64,"No game simulation is implemented here.",t.dim);}
fn exit(g:&mut Grid,t:&Theme){panel(g,t,24,7,52,13,"END SESSION");g.centered_at(10,24,52,"End this frontend session?",t.text);g.centered_at(12,24,52,"ENTER → TITLE",t.accent);g.centered_at(14,24,52,"ESC → MAIN MENU",t.dim);}

/// Build and submit the current scene as two GPU-batched meshes.
pub fn build_scene(
    commands: &mut Commands,
    theme: &Theme,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    atlas: &Cp437Atlas,
    scene: Scene,
    nav: &NavigationState,
) {
    let grid = build_grid(theme, scene, nav.selected);
    spawn_batched_grid(commands, meshes, materials, &grid, atlas);
}
