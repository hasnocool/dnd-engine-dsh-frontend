// src/input.rs
use bevy::prelude::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    None,
    Up,
    Down,
    Left,
    Right,
    Back,
    Select,
    Number(usize),
    Inventory,
    Map,
    Combat,
    Dialog,
    Next,
    Previous,
    QuitApp,
}

/// Convert physical keyboard keys into frontend navigation actions.
/// This layer intentionally contains no gameplay behavior.
pub fn read_action(keys: &bevy::prelude::ButtonInput<KeyCode>) -> Option<InputAction> {
    let action = if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        InputAction::Up
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        InputAction::Down
    } else if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        InputAction::Left
    } else if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyE) {
        InputAction::Right
    } else if keys.just_pressed(KeyCode::Escape) {
        InputAction::Back
    } else if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        InputAction::Select
    } else if keys.just_pressed(KeyCode::Digit1) || keys.just_pressed(KeyCode::Numpad1) {
        InputAction::Number(1)
    } else if keys.just_pressed(KeyCode::Digit2) || keys.just_pressed(KeyCode::Numpad2) {
        InputAction::Number(2)
    } else if keys.just_pressed(KeyCode::Digit3) || keys.just_pressed(KeyCode::Numpad3) {
        InputAction::Number(3)
    } else if keys.just_pressed(KeyCode::Digit4) || keys.just_pressed(KeyCode::Numpad4) {
        InputAction::Number(4)
    } else if keys.just_pressed(KeyCode::Digit5) || keys.just_pressed(KeyCode::Numpad5) {
        InputAction::Number(5)
    } else if keys.just_pressed(KeyCode::Digit6) || keys.just_pressed(KeyCode::Numpad6) {
        InputAction::Number(6)
    } else if keys.just_pressed(KeyCode::Digit7) || keys.just_pressed(KeyCode::Numpad7) {
        InputAction::Number(7)
    } else if keys.just_pressed(KeyCode::Digit8) || keys.just_pressed(KeyCode::Numpad8) {
        InputAction::Number(8)
    } else if keys.just_pressed(KeyCode::Digit9) || keys.just_pressed(KeyCode::Numpad9) {
        InputAction::Number(9)
    } else if keys.just_pressed(KeyCode::Digit0) || keys.just_pressed(KeyCode::Numpad0) {
        InputAction::Number(0)
    } else if keys.just_pressed(KeyCode::KeyI) {
        InputAction::Inventory
    } else if keys.just_pressed(KeyCode::KeyM) {
        InputAction::Map
    } else if keys.just_pressed(KeyCode::KeyC) {
        InputAction::Combat
    } else if keys.just_pressed(KeyCode::KeyD) {
        InputAction::Dialog
    } else if keys.just_pressed(KeyCode::Tab) || keys.just_pressed(KeyCode::PageDown) {
        InputAction::Next
    } else if keys.just_pressed(KeyCode::PageUp) {
        InputAction::Previous
    } else if keys.just_pressed(KeyCode::F10) {
        InputAction::QuitApp
    } else {
        return None;
    };

    Some(action)
}
