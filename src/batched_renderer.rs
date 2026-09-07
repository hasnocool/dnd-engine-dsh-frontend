// src/batched_renderer.rs
//! Two-draw-call renderer for the fixed 100×28 CP437 grid.
//!
//! The background layer is one indexed Mesh2d and the glyph layer is one
//! indexed Mesh2d using the runtime-generated 256-glyph IBM VGA atlas.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

use crate::cp437_grid::{Cp437Atlas, Grid, CELL_H, CELL_W, GRID_COLS, GRID_ROWS};
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

    commands.spawn((
        Mesh2d(meshes.add(background_mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::default())),
        Transform::from_xyz(0.0, 0.0, Z_BACKGROUND),
        UiRoot,
    ));

    commands.spawn((
        Mesh2d(meshes.add(glyph_mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from(atlas.image.clone()))),
        Transform::from_xyz(0.0, 0.0, Z_GLYPHS),
        UiRoot,
    ));
}

fn build_meshes(grid: &Grid) -> (Mesh, Mesh) {
    let quad_count = GRID_COLS * GRID_ROWS;
    let mut bg_positions = Vec::with_capacity(quad_count * 4);
    let mut bg_colors = Vec::with_capacity(quad_count * 4);
    let mut bg_indices = Vec::with_capacity(quad_count * 6);

    let mut glyph_positions = Vec::with_capacity(quad_count * 4);
    let mut glyph_uvs = Vec::with_capacity(quad_count * 4);
    let mut glyph_colors = Vec::with_capacity(quad_count * 4);
    let mut glyph_indices = Vec::with_capacity(quad_count * 6);

    for y in 0..GRID_ROWS {
        for x in 0..GRID_COLS {
            let cell = grid.get(x, y);
            let (left, right, bottom, top) = cell_rect(x, y);
            let base = bg_positions.len() as u32;
            bg_positions.extend_from_slice(&[
                [left, bottom, 0.0],
                [right, bottom, 0.0],
                [right, top, 0.0],
                [left, top, 0.0],
            ]);
            bg_colors.extend_from_slice(&[
                cell.bg.to_linear().to_f32_array(),
                cell.bg.to_linear().to_f32_array(),
                cell.bg.to_linear().to_f32_array(),
                cell.bg.to_linear().to_f32_array(),
            ]);
            bg_indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);

            if cell.ch == b' ' {
                continue;
            }

            let glyph_index = cell.ch as usize;
            let tile_x = (glyph_index % 32) as f32;
            let tile_y = (glyph_index / 32) as f32;
            let u0 = tile_x / 32.0;
            let v0 = tile_y / 8.0;
            let u1 = (tile_x + 1.0) / 32.0;
            let v1 = (tile_y + 1.0) / 8.0;
            let glyph_base = glyph_positions.len() as u32;
            let fg = cell.fg.to_linear().to_f32_array();

            glyph_positions.extend_from_slice(&[
                [left, bottom, 0.0],
                [right, bottom, 0.0],
                [right, top, 0.0],
                [left, top, 0.0],
            ]);
            glyph_uvs.extend_from_slice(&[
                [u0, v1],
                [u1, v1],
                [u1, v0],
                [u0, v0],
            ]);
            glyph_colors.extend_from_slice(&[fg, fg, fg, fg]);
            glyph_indices.extend_from_slice(&[
                glyph_base,
                glyph_base + 1,
                glyph_base + 2,
                glyph_base,
                glyph_base + 2,
                glyph_base + 3,
            ]);
        }
    }

    let background = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_indices(Indices::U32(bg_indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, bg_positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, bg_colors);

    let glyphs = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_indices(Indices::U32(glyph_indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, glyph_positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, glyph_uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, glyph_colors);

    (background, glyphs)
}

fn cell_rect(x: usize, y: usize) -> (f32, f32, f32, f32) {
    let left = -((GRID_COLS as f32) * CELL_W) / 2.0 + x as f32 * CELL_W;
    let right = left + CELL_W;
    let top = ((GRID_ROWS as f32) * CELL_H) / 2.0 - y as f32 * CELL_H;
    let bottom = top - CELL_H;
    (left, right, bottom, top)
}
