// src/main.rs
use bevy::prelude::*;

mod cp437_grid;
mod glyphs;
mod input;
mod pixel_ui;
mod theme;

use cp437_grid::{create_cp437_atlas, spawn_grid, Cp437Atlas};
use input::InputAction;
use pixel_ui::{build_grid, CurrentScene, NavigationState, Scene, UiRoot};
use theme::Theme;

fn main() {
    App::new()
        .insert_resource(Theme::default())
        .insert_resource(CurrentScene::default())
        .insert_resource(NavigationState::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "D&D RPG Engine — Fixed CP437 Frontend".into(),
                resolution: (1600, 900).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, navigation_system)
        .run();
}

fn setup(
    mut commands: Commands,
    theme: Res<Theme>,
    mut images: ResMut<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut nav: ResMut<NavigationState>,
) {
    commands.spawn(Camera2d);
    let atlas = create_cp437_atlas(&mut images, &mut layouts);
    commands.insert_resource(atlas.clone());
    nav.reset(Scene::Title);
    render_scene(&mut commands, &theme, &atlas, Scene::Title, &nav);
}

fn navigation_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scene: ResMut<CurrentScene>,
    mut nav: ResMut<NavigationState>,
    theme: Res<Theme>,
    atlas: Res<Cp437Atlas>,
    roots: Query<Entity, With<UiRoot>>,
) {
    let Some(action) = input::read_action(&keyboard) else { return };

    match action {
        InputAction::Up | InputAction::Previous => {
            if nav.move_up(scene.scene) {
                redraw_scene(&mut commands, &roots, &theme, &atlas, scene.scene, &nav);
            }
        }
        InputAction::Down | InputAction::Next => {
            if nav.move_down(scene.scene) {
                redraw_scene(&mut commands, &roots, &theme, &atlas, scene.scene, &nav);
            }
        }
        InputAction::Back => {
            if let Some(previous) = scene.scene.back() {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, previous);
            }
        }
        InputAction::Number(number) => {
            if let Some(target) = nav.number_target(scene.scene, number) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, target);
            }
        }
        InputAction::Inventory => {
            if matches!(scene.scene, Scene::GameBoard | Scene::Combat) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, Scene::StatsInventory);
            }
        }
        InputAction::Map => {
            if matches!(scene.scene, Scene::GameBoard | Scene::StatsInventory | Scene::Inventory) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, Scene::Map);
            }
        }
        InputAction::Combat => {
            if matches!(scene.scene, Scene::GameBoard | Scene::StatsInventory | Scene::Inventory) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, Scene::Combat);
            }
        }
        InputAction::Dialog => {
            if scene.scene == Scene::GameBoard {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, Scene::Dialog);
            }
        }
        InputAction::Left | InputAction::Right => {}
        InputAction::Select => {
            if let Some(target) = nav.selected_target(scene.scene) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, target);
            }
        }
        InputAction::QuitApp | InputAction::None => {}
    }
}

fn render_scene(
    commands: &mut Commands,
    theme: &Theme,
    atlas: &Cp437Atlas,
    scene: Scene,
    nav: &NavigationState,
) {
    commands.spawn((
        Sprite::from_color(theme.background, Vec2::new(1600.0, 900.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        UiRoot,
    ));
    let grid = build_grid(theme, scene, nav.selected);
    spawn_grid(commands, &grid, atlas, theme.background);
}

fn redraw_scene(
    commands: &mut Commands,
    roots: &Query<Entity, With<UiRoot>>,
    theme: &Theme,
    atlas: &Cp437Atlas,
    scene: Scene,
    nav: &NavigationState,
) {
    for entity in roots.iter() {
        commands.entity(entity).despawn();
    }
    render_scene(commands, theme, atlas, scene, nav);
}

fn switch_scene(
    commands: &mut Commands,
    scene: &mut CurrentScene,
    nav: &mut NavigationState,
    roots: &Query<Entity, With<UiRoot>>,
    theme: &Theme,
    atlas: &Cp437Atlas,
    next: Scene,
) {
    for entity in roots.iter() {
        commands.entity(entity).despawn();
    }
    scene.scene = next;
    nav.reset(next);
    render_scene(commands, theme, atlas, next, nav);
}
