// src/cp437_grid.rs
//! Fixed-cell CP437 renderer primitives.
//!
//! The logical screen is exactly 100 columns × 28 rows. Each cell is rendered
//! from the IBM VGA 8×16 bitmap face at a 2× scale, producing a 16×32 pixel
//! cell. The drawable area is therefore exactly 1600×896 pixels inside the
//! 1600×900 design canvas.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use neovision_core::{cp437, font};

use crate::pixel_ui::UiRoot;

pub const GRID_COLS: usize = 100;
pub const GRID_ROWS: usize = 28;
pub const GLYPH_W: f32 = 8.0;
pub const GLYPH_H: f32 = 16.0;
pub const CELL_W: f32 = 16.0;
pub const CELL_H: f32 = 32.0;
pub const GRID_PIXEL_WIDTH: f32 = GRID_COLS as f32 * CELL_W;
pub const GRID_PIXEL_HEIGHT: f32 = GRID_ROWS as f32 * CELL_H;

#[derive(Clone, Copy)]
pub struct Cell {
    pub ch: u8,
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    pub fn blank(bg: Color) -> Self {
        Self { ch: b' ', fg: Color::WHITE, bg }
    }
}

pub struct Grid {
    cells: Vec<Cell>,
    default_bg: Color,
}

impl Grid {
    pub fn new(bg: Color) -> Self {
        Self {
            cells: vec![Cell::blank(bg); GRID_COLS * GRID_ROWS],
            default_bg: bg,
        }
    }

    pub fn index(x: usize, y: usize) -> usize {
        y * GRID_COLS + x
    }

    pub fn set(&mut self, x: i32, y: i32, ch: u8, fg: Color, bg: Option<Color>) {
        if x < 0 || y < 0 || x as usize >= GRID_COLS || y as usize >= GRID_ROWS {
            return;
        }
        self.cells[Self::index(x as usize, y as usize)] = Cell {
            ch,
            fg,
            bg: bg.unwrap_or(self.default_bg),
        };
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[Self::index(x, y)]
    }

    pub fn fill(&mut self, x: usize, y: usize, w: usize, h: usize, bg: Color) {
        for yy in y..(y + h).min(GRID_ROWS) {
            for xx in x..(x + w).min(GRID_COLS) {
                self.set(xx as i32, yy as i32, b' ', Color::WHITE, Some(bg));
            }
        }
    }

    pub fn box_double(&mut self, x: usize, y: usize, w: usize, h: usize, fg: Color, bg: Color) {
        if w < 2 || h < 2 { return; }
        self.put(x, y, '╔', fg, Some(bg));
        self.put(x + w - 1, y, '╗', fg, Some(bg));
        self.put(x, y + h - 1, '╚', fg, Some(bg));
        self.put(x + w - 1, y + h - 1, '╝', fg, Some(bg));
        for xx in x + 1..x + w - 1 {
            self.put(xx, y, '═', fg, Some(bg));
            self.put(xx, y + h - 1, '═', fg, Some(bg));
        }
        for yy in y + 1..y + h - 1 {
            self.put(x, yy, '║', fg, Some(bg));
            self.put(x + w - 1, yy, '║', fg, Some(bg));
        }
    }

    pub fn box_single(&mut self, x: usize, y: usize, w: usize, h: usize, fg: Color, bg: Color) {
        if w < 2 || h < 2 { return; }
        self.put(x, y, '┌', fg, Some(bg));
        self.put(x + w - 1, y, '┐', fg, Some(bg));
        self.put(x, y + h - 1, '└', fg, Some(bg));
        self.put(x + w - 1, y + h - 1, '┘', fg, Some(bg));
        for xx in x + 1..x + w - 1 {
            self.put(xx, y, '─', fg, Some(bg));
            self.put(xx, y + h - 1, '─', fg, Some(bg));
        }
        for yy in y + 1..y + h - 1 {
            self.put(x, yy, '│', fg, Some(bg));
            self.put(x + w - 1, yy, '│', fg, Some(bg));
        }
    }

    pub fn put(&mut self, x: usize, y: usize, ch: char, fg: Color, bg: Option<Color>) {
        let byte = cp437::from_char(ch).unwrap_or_else(|| fallback_cp437(ch));
        self.set(x as i32, y as i32, byte, fg, bg);
    }

    pub fn text(&mut self, x: usize, y: usize, value: &str, fg: Color) {
        self.text_bg(x, y, value, fg, None);
    }

    pub fn text_bg(&mut self, x: usize, y: usize, value: &str, fg: Color, bg: Option<Color>) {
        for (offset, ch) in value.chars().enumerate() {
            if x + offset >= GRID_COLS { break; }
            let byte = cp437::from_char(ch).unwrap_or_else(|| fallback_cp437(ch));
            self.set((x + offset) as i32, y as i32, byte, fg, bg);
        }
    }

    pub fn centered(&mut self, y: usize, value: &str, fg: Color) {
        let width = value.chars().count().min(GRID_COLS);
        let x = (GRID_COLS.saturating_sub(width)) / 2;
        self.text(x, y, value, fg);
    }

    pub fn centered_at(&mut self, y: usize, x: usize, w: usize, value: &str, fg: Color) {
        let width = value.chars().count().min(w);
        let start = x + w.saturating_sub(width) / 2;
        self.text(start, y, value, fg);
    }

    pub fn selection(&mut self, x: usize, y: usize, w: usize, active: bool, key: &str, label: &str, t: &crate::theme::Theme) {
        let bg = if active { t.accent } else { t.panel };
        let fg = if active { t.background } else { t.text };
        self.fill(x, y, w, 1, bg);
        self.text_bg(x, y, &format!("{} {}", if active { "▶" } else { " " }, key), fg, Some(bg));
        self.text_bg(x + 5, y, label, fg, Some(bg));
    }

    pub fn hline(&mut self, x: usize, y: usize, w: usize, fg: Color) {
        for xx in x..(x + w).min(GRID_COLS) { self.put(xx, y, '─', fg, None); }
    }

    pub fn bar(&mut self, x: usize, y: usize, w: usize, filled: usize, fg: Color, bg: Color) {
        let filled = filled.min(w);
        for xx in 0..w {
            self.put(x + xx, y, if xx < filled { '█' } else { '░' }, fg, Some(bg));
        }
    }

    pub fn cells(&self) -> &[Cell] { &self.cells }
}

fn fallback_cp437(ch: char) -> u8 {
    match ch {
        '⚔' => b'+',
        '◈' => 0x04,
        '✦' => b'*',
        '♥' => 0x03,
        '♠' => 0x06,
        '♣' => 0x05,
        '♦' => 0x04,
        '†' | '‡' => b'+',
        '★' | '☆' => b'*',
        '●' => 0x07,
        '○' => 0x09,
        '■' | '□' => 0xFE,
        '◆' | '◇' => 0x04,
        '✓' => b'v',
        '✗' | '✘' => b'x',
        '⌂' => 0x7F,
        _ => b'?',
    }
}

#[derive(Resource, Clone)]
pub struct Cp437Atlas {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn create_cp437_atlas(
    images: &mut Assets<Image>,
    layouts: &mut Assets<TextureAtlasLayout>,
) -> Cp437Atlas {
    const ATLAS_W: usize = 32 * 8;
    const ATLAS_H: usize = 8 * 16;

    let mut data = vec![0u8; ATLAS_W * ATLAS_H * 4];
    for glyph_index in 0..256usize {
        let glyph = font::glyph(glyph_index as u8);
        let tile_x = (glyph_index % 32) * 8;
        let tile_y = (glyph_index / 32) * 16;
        for row in 0..16usize {
            for col in 0..8usize {
                let on = (glyph[row] & (0x80 >> col)) != 0;
                let dst = ((tile_y + row) * ATLAS_W + tile_x + col) * 4;
                data[dst] = 255;
                data[dst + 1] = 255;
                data[dst + 2] = 255;
                data[dst + 3] = if on { 255 } else { 0 };
            }
        }
    }

    let mut image = Image::new_fill(
        Extent3d { width: ATLAS_W as u32, height: ATLAS_H as u32, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        mag_filter: ImageFilterMode::Nearest,
        min_filter: ImageFilterMode::Nearest,
        mipmap_filter: ImageFilterMode::Nearest,
        ..ImageSamplerDescriptor::linear()
    });

    let image_handle = images.add(image);
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(8, 16), 32, 8, None, None,
    ));

    Cp437Atlas { image: image_handle, layout }
}

pub fn spawn_grid(commands: &mut Commands, grid: &Grid, atlas: &Cp437Atlas, page_bg: Color) {
    for y in 0..GRID_ROWS {
        for x in 0..GRID_COLS {
            let cell = grid.get(x, y);
            let px = -GRID_PIXEL_WIDTH / 2.0 + x as f32 * CELL_W + CELL_W / 2.0;
            let py = GRID_PIXEL_HEIGHT / 2.0 - y as f32 * CELL_H - CELL_H / 2.0 + 2.0;

            if cell.bg != page_bg {
                commands.spawn((
                    Sprite::from_color(cell.bg, Vec2::new(CELL_W, CELL_H)),
                    Transform::from_xyz(px, py, 1.0),
                    UiRoot,
                ));
            }

            if cell.ch != b' ' {
                let mut sprite = Sprite::from_atlas_image(
                    atlas.image.clone(),
                    TextureAtlas { layout: atlas.layout.clone(), index: cell.ch as usize },
                );
                sprite.color = cell.fg;
                commands.spawn((
                    sprite,
                    Transform::from_xyz(px, py, 2.0).with_scale(Vec3::splat(2.0)),
                    UiRoot,
                ));
            }
        }
    }
}
