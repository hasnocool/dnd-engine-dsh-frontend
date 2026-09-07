// src/main.rs
use bevy::prelude::*;

mod batched_renderer;
mod cp437_grid;
mod glyphs;
mod input;
mod pixel_ui;
mod theme;

use cp437_grid::{create_cp437_atlas, Cp437Atlas};
use input::InputAction;
use pixel_ui::{build_scene, CurrentScene, NavigationState, Scene, UiRoot};
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut nav: ResMut<NavigationState>,
) {
    commands.spawn(Camera2d);
    let atlas = create_cp437_atlas(&mut images, &mut layouts);
    commands.insert_resource(atlas.clone());
    nav.reset(Scene::Title);
    build_scene(&mut commands, &theme, &mut meshes, &mut materials, &atlas, Scene::Title, &nav);
}

fn navigation_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scene: ResMut<CurrentScene>,
    mut nav: ResMut<NavigationState>,
    theme: Res<Theme>,
    atlas: Res<Cp437Atlas>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    roots: Query<Entity, With<UiRoot>>,
) {
    let Some(action) = input::read_action(&keyboard) else { return };

    match action {
        InputAction::Up | InputAction::Previous => {
            if nav.move_up(scene.scene) {
                redraw_scene(&mut commands, &roots, &theme, &atlas, &mut meshes, &mut materials, scene.scene, &nav);
            }
        }
        InputAction::Down | InputAction::Next => {
            if nav.move_down(scene.scene) {
                redraw_scene(&mut commands, &roots, &theme, &atlas, &mut meshes, &mut materials, scene.scene, &nav);
            }
        }
        InputAction::Back => {
            if let Some(previous) = scene.scene.back() {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, &mut meshes, &mut materials, previous);
            }
        }
        InputAction::Number(number) => {
            if let Some(target) = nav.number_target(scene.scene, number) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, &mut meshes, &mut materials, target);
            }
        }
        InputAction::Inventory => {
            if matches!(scene.scene, Scene::GameBoard | Scene::Combat) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, &mut meshes, &mut materials, Scene::StatsInventory);
            }
        }
        InputAction::Map => {
            if matches!(scene.scene, Scene::GameBoard | Scene::StatsInventory | Scene::Inventory) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, &mut meshes, &mut materials, Scene::Map);
            }
        }
        InputAction::Combat => {
            if matches!(scene.scene, Scene::GameBoard | Scene::StatsInventory | Scene::Inventory) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, &mut meshes, &mut materials, Scene::Combat);
            }
        }
        InputAction::Dialog => {
            if scene.scene == Scene::GameBoard {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, &mut meshes, &mut materials, Scene::Dialog);
            }
        }
        InputAction::Left | InputAction::Right => {}
        InputAction::Select => {
            if let Some(target) = nav.selected_target(scene.scene) {
                switch_scene(&mut commands, &mut scene, &mut nav, &roots, &theme, &atlas, &mut meshes, &mut materials, target);
            }
        }
        InputAction::QuitApp | InputAction::None => {}
    }
}

fn redraw_scene(
    commands: &mut Commands,
    roots: &Query<Entity, With<UiRoot>>,
    theme: &Theme,
    atlas: &Cp437Atlas,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    scene: Scene,
    nav: &NavigationState,
) {
    for entity in roots.iter() {
        commands.entity(entity).despawn();
    }
    build_scene(commands, theme, meshes, materials, atlas, scene, nav);
}

fn switch_scene(
    commands: &mut Commands,
    scene: &mut CurrentScene,
    nav: &mut NavigationState,
    roots: &Query<Entity, With<UiRoot>>,
    theme: &Theme,
    atlas: &Cp437Atlas,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    next: Scene,
) {
    for entity in roots.iter() {
        commands.entity(entity).despawn();
    }
    scene.scene = next;
    nav.reset(next);
    build_scene(commands, theme, meshes, materials, atlas, next, nav);
}
