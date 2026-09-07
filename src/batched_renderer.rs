// src/batched_renderer.rs
//! Two-draw-call renderer for the fixed 100×28 CP437 grid.
//!
//! The background layer is one indexed Mesh2d and the glyph layer is one
//! indexed Mesh2d using the runtime-generated 256-glyph IBM VGA atlas.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::render_resource::Indices;

use crate::cp437_grid::{Cp437Atlas, Grid, GRID_COLS, GRID_ROWS, CELL_H, CELL_W};
use crate::pixel_ui::UiRoot;

const Z_BACKGROUND: f32 = 0.0;
const Z_GLYPHS: f32 = 1.0;

pub fn spawn_batched_grid(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    grid: &Grid,
    atlas: &Cp437Atlas,
) {
    let (background_mesh, glyph_mesh) = build_meshes(grid);

    let background_handle = meshes.add(background_mesh);
    commands.spawn((
        Mesh2d(background_handle),
        MeshMaterial2d(materials.add(ColorMaterial::default())),
        Transform::from_xyz(0.0, 0.0, Z_BACKGROUND),
        UiRoot,
    ));

    let glyph_handle = meshes.add(glyph_mesh);
    commands.spawn((
        Mesh2d(glyph_handle),
        MeshMaterial2d(materials.add(ColorMaterial::from(atlas.image.clone()))),
        Transform::from_xyz(0.0, 0.0, Z_GLYPHS),
        UiRoot,
    ));
}

fn build_meshes(grid: &Grid) -> (Mesh, Mesh) {
    let mut bg_positions: Vec<[f32; 3]> = Vec::with_capacity(GRID_COLS * GRID_ROWS * 4);
    let mut bg_colors: Vec<[f32; 4]> = Vec::with_capacity(GRID_COLS * GRID_ROWS * 4);
    let mut bg_indices: Vec<u32> = Vec::with_capacity(GRID_COLS * GRID_ROWS * 6);

    let mut glyph_positions: Vec<[f32; 3]> = Vec::with_capacity(GRID_COLS * GRID_ROWS * 4);
    let mut glyph_uvs: Vec<[f32; 2]> = Vec::with_capacity(GRID_COLS * GRID_ROWS * 4);
    let mut glyph_colors: Vec<[f32; 4]> = Vec::with_capacity(GRID_COLS * GRID_ROWS * 4);
    let mut glyph_indices: Vec<u32> = Vec::with_capacity(GRID_COLS * GRID_ROWS * 6);

    for y in 0..GRID_ROWS {
        for x in 0..GRID_COLS {
            let cell = grid.get(x, y);
            let (left, right, bottom, top) = cell_rect(x, y);
            let bg_color = cell.bg.to_linear().to_f32_array();
            let fg_color = cell.fg.to_linear().to_f32_array();

            push_quad(
                &mut bg_positions,
                &mut bg_indices,
                left, right, bottom, top,
                [0.0, 0.0],
                &bg_color,
                bg_positions.len() as u32,
            );

            if cell.ch != b' ' {
                let glyph_index = cell.ch as usize;
                let tile_x = (glyph_index % 32) as f32;
                let tile_y = (glyph_index / 32) as f32;
                let u0 = tile_x / 32.0;
                let v0 = tile_y / 8.0;
                let u1 = (tile_x + 1.0) / 32.0;
                let v1 = (tile_y + 1.0) / 8.0;

                push_glyph_quad(
                    &mut glyph_positions,
                    &mut glyph_uvs,
                    &mut glyph_colors,
                    &mut glyph_indices,
                    left, right, bottom, top,
                    u0, v0, u1, v1,
                    &fg_color,
                    glyph_positions.len() as u32,
                );
            }
        }
    }

    let bg_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_indices(Indices::U32(bg_indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, bg_positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, bg_colors);

    let glyph_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_indices(Indices::U32(glyph_indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, glyph_positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, glyph_uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, glyph_colors);

    (bg_mesh, glyph_mesh)
}

fn cell_rect(x: usize, y: usize) -> (f32, f32, f32, f32) {
    let left = -((GRID_COLS as f32) * CELL_W) / 2.0 + x as f32 * CELL_W;
    let right = left + CELL_W;
    let top = ((GRID_ROWS as f32) * CELL_H) / 2.0 - y as f32 * CELL_H;
    let bottom = top - CELL_H;
    (left, right, bottom, top)
}

fn push_quad(
    positions: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    _unused_uv: [f32; 2],
    _color: &[f32; 4],
    base: u32,
) {
    positions.extend_from_slice(&[
        [left, bottom, 0.0],
        [right, bottom, 0.0],
        [right, top, 0.0],
        [left, top, 0.0],
    ]);
    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}

fn push_glyph_quad(
    positions: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    colors: &mut Vec<[f32; 4]>,
    indices: &mut Vec<u32>,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    u0: f32,
    v0: f32,
    u1: f32,
    v1: f32,
    color: &[f32; 4],
    base: u32,
) {
    positions.extend_from_slice(&[
        [left, bottom, 0.0],
        [right, bottom, 0.0],
        [right, top, 0.0],
        [left, top, 0.0],
    ]);
    uvs.extend_from_slice(&[[u0, v1], [u1, v1], [u1, v0], [u0, v0]]);
    colors.extend_from_slice(&[*color, *color, *color, *color]);
    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}
