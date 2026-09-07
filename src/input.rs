// src/input.rs
use bevy::prelude::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    None,
    Up,
    Down,
    Back,
    Select,
    Number(usize),
    Inventory,
    Map,
    Combat,
    Dialog,
    QuitApp,
}

pub fn read_action(keys: &bevy::prelude::ButtonInput<KeyCode>) -> Option<InputAction> {
    let action = if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        InputAction::Up
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        InputAction::Down
    } else if keys.just_pressed(KeyCode::Escape) {
        InputAction::Back
    } else if keys.just_pressed(KeyCode::Enter) {
        InputAction::Select
    } else if keys.just_pressed(KeyCode::Digit1) { InputAction::Number(1)
    } else if keys.just_pressed(KeyCode::Digit2) { InputAction::Number(2)
    } else if keys.just_pressed(KeyCode::Digit3) { InputAction::Number(3)
    } else if keys.just_pressed(KeyCode::Digit4) { InputAction::Number(4)
    } else if keys.just_pressed(KeyCode::Digit5) { InputAction::Number(5)
    } else if keys.just_pressed(KeyCode::Digit6) { InputAction::Number(6)
    } else if keys.just_pressed(KeyCode::Digit0) { InputAction::Number(0)
    } else if keys.just_pressed(KeyCode::KeyI) { InputAction::Inventory
    } else if keys.just_pressed(KeyCode::KeyM) { InputAction::Map
    } else if keys.just_pressed(KeyCode::KeyC) { InputAction::Combat
    } else if keys.just_pressed(KeyCode::KeyD) { InputAction::Dialog
    } else if keys.just_pressed(KeyCode::F10) { InputAction::QuitApp
    } else { return None };

    Some(action)
}
