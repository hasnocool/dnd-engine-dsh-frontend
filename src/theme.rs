// src/theme.rs
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Theme {
    pub background: Color,
    pub panel: Color,
    pub border: Color,
    pub primary: Color,
    pub accent: Color,
    pub success: Color,
    pub danger: Color,
    pub warning: Color,
    pub text: Color,
    pub dim: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::srgb(0.006, 0.014, 0.022),
            panel: Color::srgb(0.012, 0.030, 0.046),
            border: Color::srgb(0.20, 0.68, 0.96),
            primary: Color::srgb(0.00, 0.68, 0.96),
            accent: Color::srgb(1.00, 0.74, 0.16),
            success: Color::srgb(0.18, 0.90, 0.34),
            danger: Color::srgb(1.00, 0.16, 0.20),
            warning: Color::srgb(1.00, 0.58, 0.10),
            text: Color::srgb(0.88, 0.95, 0.98),
            dim: Color::srgb(0.42, 0.56, 0.64),
        }
    }
}
