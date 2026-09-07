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
    Title, MainMenu, CharacterCreation, GameBoard, StatsInventory, Map, Inventory, Combat, Dialog, Settings, Credits, Exit,
}

#[derive(Clone, Copy)]
pub struct NavItem { pub label: &'static str, pub target: Scene, pub hotkey: &'static str }

impl Scene {
    pub fn title(self) -> &'static str {
        match self {
            Scene::Title => "D&D RPG ENGINE", Scene::MainMenu => "MAIN MENU", Scene::CharacterCreation => "CREATE YOUR HERO",
            Scene::GameBoard => "MAIN GAME BOARD", Scene::StatsInventory => "STATS & INVENTORY", Scene::Map => "MAP / WORLD VIEW",
            Scene::Inventory => "INVENTORY", Scene::Combat => "COMBAT / BATTLE", Scene::Dialog => "DIALOG / NPC INTERACTION",
            Scene::Settings => "SETTINGS", Scene::Credits => "CREDITS", Scene::Exit => "END SESSION",
        }
    }

    pub fn back(self) -> Option<Self> {
        match self {
            Scene::Title => None,
            Scene::MainMenu => Some(Scene::Title),
            Scene::CharacterCreation => Some(Scene::MainMenu),
            Scene::GameBoard => Some(Scene::MainMenu),
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
            NavItem { label: "Attack", target: Scene::GameBoard, hotkey: "1" }, NavItem { label: "Defend", target: Scene::GameBoard, hotkey: "2" },
            NavItem { label: "Skills", target: Scene::GameBoard, hotkey: "3" }, NavItem { label: "Items", target: Scene::Inventory, hotkey: "4" },
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

pub fn build_scene(commands: &mut Commands, theme: &Theme, _assets: &AssetServer, scene: Scene, nav: &NavigationState) {
    commands.spawn((
        Node { width: Val::Percent(100.0), height: Val::Percent(100.0), padding: UiRect::all(Val::Px(18.0)), flex_direction: FlexDirection::Column, row_gap: Val::Px(8.0), overflow: Overflow::clip(), ..default() },
        BackgroundColor(theme.background), UiRoot,
    )).with_children(|page| {
        text(page, &format!("╔{}╗", "═".repeat(108)), 16.0, theme.accent);
        text(page, &format!("║{:^108}║", scene.title()), 24.0, theme.text);
        text(page, &format!("╚{}╝", "═".repeat(108)), 16.0, theme.accent);
        match scene {
            Scene::Title => title(page, theme), Scene::MainMenu => main_menu(page, theme, nav.selected),
            Scene::CharacterCreation => creation(page, theme, nav.selected), Scene::GameBoard => board(page, theme, nav.selected),
            Scene::StatsInventory => stats(page, theme), Scene::Map => map(page, theme), Scene::Inventory => inventory(page, theme),
            Scene::Combat => combat(page, theme, nav.selected), Scene::Dialog => dialog(page, theme, nav.selected),
            Scene::Settings => settings(page, theme, nav.selected), Scene::Credits => credits(page, theme), Scene::Exit => exit(page, theme),
        }
        spacer(page, 4.0);
        text(page, "↑↓ / W S Navigate   ENTER Select   0-9 Quick Select   ESC Back   I Inventory   M Map   C Combat   D Dialogue", 13.0, theme.dim);
        text(page, "D&D RPG ENGINE  ·  CP437 + UNICODE TEXTMODE  ·  FRONTEND WIREFRAME  ·  NAVIGATION ONLY", 11.0, theme.dim);
    });
}

fn title(p: &mut ChildSpawner, t: &Theme) {
    hrow(p, |r| {
        panel(r, t, 42.0, |q| {
            text(q, "             ╔══════════════════╗", 17.0, t.primary);
            text(q, "             ║    D&D RPG       ║", 22.0, t.text);
            text(q, "             ║     ENGINE       ║", 22.0, t.text);
            text(q, "             ╚══════════════════╝", 17.0, t.primary);
            spacer(q, 10.0); text(q, "\"Not all those who wander are lost…\"", 17.0, t.text);
            spacer(q, 16.0); text(q, "▶  NEW GAME", 21.0, t.accent); text(q, "   CONTINUE", 21.0, t.text); text(q, "   SETTINGS", 21.0, t.text); text(q, "   QUIT", 21.0, t.text);
        });
        panel(r, t, 58.0, |q| {
            text(q, "                 ▲", 18.0, t.primary); text(q, "                ╱ ╲", 18.0, t.primary);
            text(q, "          ╱╲   ╱███╲   ╱╲", 18.0, t.primary); text(q, "         ╱██╲ ╱█████╲ ╱██╲", 18.0, t.primary);
            text(q, "        ║████║║█████║║████║", 18.0, t.primary); text(q, "             ═══╩═══", 18.0, t.dim);
            spacer(q, 10.0); text(q, "★  Adventure awaits…  ★", 17.0, t.accent); text(q, "⚔  Explore · Fight · Discover · Survive", 15.0, t.text);
        });
    });
}

fn main_menu(p: &mut ChildSpawner, t: &Theme, selected: usize) {
    hrow(p, |r| {
        panel(r, t, 45.0, |q| { text(q, "MAIN MENU", 21.0, t.text); spacer(q, 7.0); for (i, (k,l)) in [("1","New Adventure"),("2","Continue"),("3","Load Game"),("4","Settings"),("5","Credits"),("6","Quit")].iter().enumerate() { selected_line(q,t,selected==i,k,l); } });
        panel(r,t,55.0,|q| { text(q,"                ╱╲",18.0,t.primary); text(q,"          ╱╲   ╱██╲   ╱╲",18.0,t.primary); text(q,"         ║████║║████║║████║",18.0,t.primary); spacer(q,10.0); text(q,"        ★  Adventure awaits…  ★",17.0,t.accent); });
    });
}

fn creation(p: &mut ChildSpawner, t: &Theme, selected: usize) {
    hrow(p,|r| {
        panel(r,t,31.0,|q| { text(q,"Create Your Hero",19.0,t.text); for (i,l) in ["Race","Class","Background","Abilities","Appearance","Name"].iter().enumerate(){selected_line(q,t,selected==i,"▶",l);} spacer(q,8.0); text(q,"[7] NEXT  →  ENTER",14.0,t.accent); });
        panel(r,t,47.0,|q| { text(q,"Select Race",19.0,t.text); text(q,"┌──────────────────────────────┐",14.0,t.border); for l in ["Human","Elf","Dwarf","Halfling","Dragonborn","Tiefling","Half-Elf","Half-Orc"]{text(q,&format!("│ {:<28} │",l),15.0,t.text);} text(q,"└──────────────────────────────┘",14.0,t.border); spacer(q,6.0); text(q,"Human",17.0,t.accent); text(q,"Versatile and ambitious.",14.0,t.text); text(q,"+1 STR   +1 CON",15.0,t.success); });
        panel(r,t,22.0,|q| { text(q,"     ╱╲",16.0,t.primary); text(q,"    ╱██╲",16.0,t.primary); text(q,"   ║████║",16.0,t.primary); text(q,"    ╲██╱",16.0,t.primary); text(q,"STR 14",14.0,t.text); text(q,"DEX 12",14.0,t.text); text(q,"CON 13",14.0,t.text); text(q,"INT 10",14.0,t.text); text(q,"WIS 11",14.0,t.text); text(q,"CHA 08",14.0,t.text); });
    });
}

fn board(p: &mut ChildSpawner,t:&Theme,selected:usize){
    hrow(p,|r|{panel(r,t,70.0,|q|{text(q,"OLD MINES — LEVEL 2",16.0,t.accent); for l in dungeon(){text(q,l,11.0,t.text);} text(q,"▶ You see a goblin (Lv 2)",14.0,t.warning);}); panel(r,t,30.0,|q|{text(q,"ARTHAS  Lv 3  Fighter",17.0,t.text); text(q,"HP  ██████████████░░ 28/28",13.0,t.success); text(q,"MP  ██████████░░░░░░ 10/10",13.0,t.primary); text(q,"XP  ███████░░░░░░░░ 34/100",13.0,t.accent); spacer(q,6.0); for (i,(k,l)) in [(0,"[I] Inventory / Stats"),(1,"[M] World Map"),(2,"[C] Combat"),(3,"[D] Dialogue")].iter(){selected_line(q,t,selected==*i,"▶",l);} });});
}

fn stats(p:&mut ChildSpawner,t:&Theme){hrow(p,|r|{panel(r,t,47.0,|q|{text(q,"CHARACTER STATS",17.0,t.text); for l in ["STR 14 (+2)","DEX 12 (+1)","CON 13 (+1)","INT 10 (+0)","WIS 11 (+0)","CHA 08 (-1)","Saving Throws  Fort +4  Ref +2  Will +1","Skills  Athletics +4  Stealth +3  Perception +1"]{text(q,l,14.0,t.text);}}); panel(r,t,53.0,|q|{text(q,"INVENTORY",17.0,t.text); for l in ["⚔ Longsword                 (e)","◈ Wooden Shield              (e)","♥ Healing Potion x3","◇ Rations x5","† Torch x4","★ Gold 125"]{text(q,l,14.0,t.text);} text(q,"Weight: 12 / 50",14.0,t.dim);});});}

fn map(p:&mut ChildSpawner,t:&Theme){hrow(p,|r|{panel(r,t,78.0,|q|{text(q,"WORLD MAP",17.0,t.text); for l in world(){text(q,l,11.0,t.text);}}); panel(r,t,22.0,|q|{text(q,"LEGEND",16.0,t.text); for l in ["@ Player","⌂ Town","☠ Dungeon","▲ Forest","╱╲ Mountains","≈≈ Water","✦ Point of Interest"]{text(q,l,13.0,t.text);}});});}

fn inventory(p:&mut ChildSpawner,t:&Theme){hrow(p,|r|{panel(r,t,61.0,|q|{text(q,"INVENTORY │ EQUIPMENT │ KEY ITEMS │ LOOT",15.0,t.accent); for l in ["⚔ Longsword       1   5.0 lb   50 gp","◈ Wooden Shield    1   6.0 lb   40 gp","♥ Healing Potion   3   0.5 lb   25 gp","◇ Rations          5   0.2 lb    5 gp","† Torch            4   0.1 lb    2 gp","★ Gold           125        125 gp"]{text(q,l,13.0,t.text);} }); panel(r,t,39.0,|q|{text(q,"⚔ LONGSWORD",17.0,t.accent); text(q,"A well-crafted longsword.",14.0,t.text); text(q,"Damage: 1d8+2",14.0,t.danger); text(q,"Weight: 5.0",14.0,t.text); text(q,"Value: 50",14.0,t.accent);});});}

fn combat(p:&mut ChildSpawner,t:&Theme,selected:usize){hrow(p,|r|{panel(r,t,27.0,|q|{text(q,"GOBLIN",17.0,t.success); text(q,"        ╲▲╱",17.0,t.success); text(q,"       ╔███╗",17.0,t.success); text(q,"HP ██████████ 12/12",12.0,t.danger);}); panel(r,t,48.0,|q|{text(q,"⚔ COMBAT / BATTLE ⚔",20.0,t.accent); for (i,l) in ["ATTACK","DEFEND","SKILLS","ITEMS","FLEE"].iter().enumerate(){selected_line(q,t,selected==i,&format!("[{}]",i+1),l);}}); panel(r,t,25.0,|q|{text(q,"ARTHAS Lv 3",17.0,t.text); text(q,"HP ██████████████",13.0,t.success); text(q,"28 / 28",13.0,t.text);});}); panel_full(p,t,|q|{text(q,"▶ Goblin attacks for 3 damage!",13.0,t.danger); text(q,"▶ Your turn.",13.0,t.accent);});}

fn dialog(p:&mut ChildSpawner,t:&Theme,selected:usize){panel_full(p,t,|q|{text(q,"Elder Seer",18.0,t.accent); text(q,"\"The path you seek is not easy, young one. To the east lies",15.0,t.text); text(q," a dungeon filled with ancient dangers. Are you prepared?\"",15.0,t.text); spacer(q,8.0); for (i,l) in ["I am ready.","Tell me more.","Leave."].iter().enumerate(){selected_line(q,t,selected==i,&format!("{}.",i+1),l);}});}

fn settings(p:&mut ChildSpawner,t:&Theme,selected:usize){panel_full(p,t,|q|{text(q,"SETTINGS",19.0,t.text); for (i,l) in ["Audio","Display","Controls","Back"].iter().enumerate(){selected_line(q,t,selected==i,&format!("[{}]",if i==3{0}else{i+1}),l);} spacer(q,8.0); text(q,"Audio     ████████████ 80%",14.0,t.text); text(q,"Display   1600×900 · 16:9 · Terminal Mode",14.0,t.text); text(q,"Controls  W A S D · Arrows · Enter · Esc",14.0,t.text);});}

fn credits(p:&mut ChildSpawner,t:&Theme){panel_full(p,t,|q|{text(q,"D&D RPG ENGINE FRONTEND",22.0,t.accent); text(q,"Rust · Bevy · Winit",17.0,t.text); text(q,"CP437-inspired Unicode / ANSI textmode presentation",14.0,t.text); spacer(q,8.0); for l in ["✓ Scene navigation","✓ Keyboard hotkeys","✓ 16:9 terminal UI system","✓ Dense box / block / symbol glyphs"]{text(q,l,14.0,t.success);}});}

fn exit(p:&mut ChildSpawner,t:&Theme){panel_full(p,t,|q|{text(q,"☠ END SESSION ☠",22.0,t.danger); text(q,"ENTER → Return to Title",15.0,t.accent); text(q,"ESC   → Return to Main Menu",15.0,t.dim);});}

fn hrow(parent:&mut ChildSpawner,build:impl FnOnce(&mut ChildSpawner)){parent.spawn(Node{width:Val::Percent(100.0),flex_direction:FlexDirection::Row,column_gap:Val::Px(10.0),flex_grow:1.0,min_height:Val::Px(0.0),..default()}).with_children(build);}

fn panel(parent:&mut ChildSpawner,t:&Theme,width:f32,build:impl FnOnce(&mut ChildSpawner)){parent.spawn((Node{width:Val::Percent(width),height:Val::Percent(100.0),padding:UiRect::all(Val::Px(10.0)),flex_direction:FlexDirection::Column,border:UiRect::all(Val::Px(1.0)),overflow:Overflow::clip(),..default()},BorderColor::all(t.border),BackgroundColor(t.panel))).with_children(build);}

fn panel_full(parent:&mut ChildSpawner,t:&Theme,build:impl FnOnce(&mut ChildSpawner)){parent.spawn((Node{width:Val::Percent(100.0),flex_grow:1.0,padding:UiRect::all(Val::Px(12.0)),flex_direction:FlexDirection::Column,border:UiRect::all(Val::Px(1.0)),overflow:Overflow::clip(),..default()},BorderColor::all(t.border),BackgroundColor(t.panel))).with_children(build);}

fn selected_line(p:&mut ChildSpawner,t:&Theme,selected:bool,key:&str,label:&str){text(p,&format!("{} {:<4} {}",if selected{"▶"}else{" "},key,label),16.0,if selected{t.accent}else{t.text});}
fn spacer(p:&mut ChildSpawner,h:f32){p.spawn(Node{height:Val::Px(h),width:Val::Percent(100.0),flex_shrink:0.0,..default()});}
fn text(p:&mut ChildSpawner,s:&str,size:f32,color:Color){p.spawn((Text::new(s),TextFont{font:FontSource::Monospace,font_size:FontSize::Px(size),..default()},TextColor(color),TextLayout::new_with_justify(Justify::Left)));}

fn dungeon()->Vec<&'static str>{vec!["┌────────────────────────────────────────────────────────────┐","│ █████████     ░░░░░░░░     ██████████████████████████████ │","│ █ @     █─────░    ◇ ░─────█                 ☠           █ │","│ █       █     ░   g  ░     █                           █ │","│ █████████     ░░░░░░░░     ████████████  ██████████████ │","│      ╲           ┌────────────┐              !          │","│       ╲──────────│     $      │─────────────────╲      │","│                   └────────────┘                  ╲     │","│                         ╔══════╗                 ╲     │","│                         ║ CHEST║                  ╲    │","│                         ╚══════╝                   ≈≈≈ │","└────────────────────────────────────────────────────────────┘"]}
fn world()->Vec<&'static str>{vec!["┌──────────────────────────────────────────────────────────────┐","│ ▲▲▲▲▲▲▲▲▲▲▲▲      ╱╲       ▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲ │","│ ▲▲  ⌂ TOWN ▲▲    ╱██╲      ▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲ │","│ ▲▲▲▲▲▲▲▲▲▲▲▲   ╱████╲───────╲       ✦               │","│       ╲────────╱██████╲───────╲──────────☠ DUNGEON    │","│        ╲       ╲████╱        @                         │","│ ▲▲▲▲▲▲▲╲════════║═══════════╝▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲ │","│ ≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈ │","└──────────────────────────────────────────────────────────────┘"]}
