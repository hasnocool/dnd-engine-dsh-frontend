// src/main.rs
use bevy::prelude::*;

mod input;
mod theme;
mod ui;

use input::InputAction;
use theme::Theme;
use ui::{build_scene, CurrentScene, NavigationState, Scene, UiRoot};

fn main() {
    App::new()
        .insert_resource(Theme::default())
        .insert_resource(CurrentScene::default())
        .insert_resource(NavigationState::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "D&D RPG Engine — CP437 / Unicode Frontend".into(),
                resolution: (1600, 900).into(),
                resizable: true,
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
    assets: Res<AssetServer>,
    mut nav: ResMut<NavigationState>,
) {
    commands.spawn(Camera2d);
    nav.reset(Scene::Title);
    build_scene(&mut commands, &theme, &assets, Scene::Title, &nav);
}

fn navigation_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scene: ResMut<CurrentScene>,
    mut nav: ResMut<NavigationState>,
    theme: Res<Theme>,
    assets: Res<AssetServer>,
    roots: Query<Entity, With<UiRoot>>,
) {
    let Some(action) = input::read_action(&keyboard) else { return };

    match action {
        InputAction::Up | InputAction::Previous => {
            if nav.move_up(scene.scene) {
                redraw_scene(&mut commands, &roots, &theme, &assets, scene.scene, &nav);
            }
        }
        InputAction::Down | InputAction::Next => {
            if nav.move_down(scene.scene) {
                redraw_scene(&mut commands, &roots, &theme, &assets, scene.scene, &nav);
            }
        }
        InputAction::Back => {
            if let Some(previous) = scene.scene.back() {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &assets, previous);
            }
        }
        InputAction::Number(number) => {
            if let Some(target) = nav.number_target(scene.scene, number) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &assets, target);
            }
        }
        InputAction::Inventory => {
            if matches!(scene.scene, Scene::GameBoard | Scene::Combat) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &assets, Scene::StatsInventory);
            }
        }
        InputAction::Map => {
            if matches!(scene.scene, Scene::GameBoard | Scene::StatsInventory | Scene::Inventory) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &assets, Scene::Map);
            }
        }
        InputAction::Combat => {
            if matches!(scene.scene, Scene::GameBoard | Scene::StatsInventory | Scene::Inventory) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &assets, Scene::Combat);
            }
        }
        InputAction::Dialog => {
            if scene.scene == Scene::GameBoard {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &assets, Scene::Dialog);
            }
        }
        InputAction::Left | InputAction::Right => {
            // Reserved for future horizontal tab/character-creation navigation.
            // This prototype intentionally does not mutate any game state.
        }
        InputAction::Select => {
            if let Some(target) = nav.selected_target(scene.scene) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &assets, target);
            }
        }
        InputAction::QuitApp | InputAction::None => {
            // Deliberately no application quit/gameplay side effect in the wireframe prototype.
        }
    }
}

fn redraw_scene(
    commands: &mut Commands,
    roots: &Query<Entity, With<UiRoot>>,
    theme: &Theme,
    assets: &AssetServer,
    scene: Scene,
    nav: &NavigationState,
) {
    for entity in roots.iter() {
        commands.entity(entity).despawn();
    }
    build_scene(commands, theme, assets, scene, nav);
}

fn switch_scene(
    commands: &mut Commands,
    scene: &mut CurrentScene,
    nav: &mut NavigationState,
    roots: &Query<Entity, With<UiRoot>>,
    theme: &Theme,
    assets: &AssetServer,
    next: Scene,
) {
    for entity in roots.iter() {
        commands.entity(entity).despawn();
    }
    scene.scene = next;
    nav.reset(next);
    build_scene(commands, theme, assets, next, nav);
}
