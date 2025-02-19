use super::{chunk::{Chunk, CHUNK_SIZE}, common::Position, tile::*};
use crate::math::matrix::Matrix;

use bevy::prelude::*;

/// The board size measured in chunks
/// Total board size is BOARD_SIZE * CHUNK_SIZE
pub const BOARD_SIZE: usize = 8;

/// BoardLayer is an abstract representation of a layer of the game board.
/// It doesn't exist in game, but instead determines how the tiles will be
/// spawned during setup.
#[derive(Default, Clone)]
pub struct BoardLayer {
    pub chunks: Matrix<Chunk>,
    pub z_index: usize,
}

impl BoardLayer {
    pub fn new(z_index: usize) -> Self {
        let chunks: Matrix<Chunk> = Matrix::new(BOARD_SIZE, BOARD_SIZE);

        BoardLayer { chunks, z_index }
    }
}

#[derive(Resource, Clone)]
pub struct Board {
    pub layers: Vec<BoardLayer>,
}

impl Board {
    pub fn push_layer(&mut self, layer: BoardLayer) {
        self.layers.push(layer);
    }
}

impl Default for Board {
    fn default() -> Self {
        let layer = BoardLayer::new(0);
        let mut board = Board { layers: vec![] };
        board.push_layer(layer);
        board
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, board_setup_system.run_if(resource_changed::<Board>));
    }
}

/// Spawns all BoardLayers one by one
fn board_setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
) {
    let tile_handle: Handle<Image> = asset_server.load("graphics/tile.png");

    for chunk_row in 0..BOARD_SIZE {
        for chunk_col in 0..BOARD_SIZE {
            let mut chunk = Chunk::default();
            let mesh_handle = chunk.build_chunk_mesh(&mut meshes);

            // Compute the chunk’s world position.
            // Each chunk covers CHUNK_SIZE * TILE_SIZE units.
            let x = (chunk_col * CHUNK_SIZE * TILE_SIZE) as f32;
            let y = (chunk_row * CHUNK_SIZE * TILE_SIZE) as f32;

            // commands.spawn((
            //     chunk, // Store the chunk data if you need to update it later.
            //     Mesh2d(mesh_handle),
            //     // Optionally, add a material that uses your tile_texture.
            //     // This example assumes your material/shader knows how to use the UVs.
            // ));
        }
    }
}

/// Spawn all tiles of a single BoardLayer as Entities
fn spawn_layer(
    commands: &mut Commands,
    tile_handle: &Handle<Image>,
    layer: &BoardLayer,
    offset: Vec2,
) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            // if let Some(ref _tile) = layer.chunks.get(row, col).unwrap() {
            //     commands.spawn((
            //         Tile,
            //         Position { x: col, y: row },
            //         WalkSpeed(1.0),
            //         Sprite {
            //             image: tile_handle.clone(),
            //             ..default()
            //         },
            //         Transform {
            //             translation: Vec3 {
            //                 x: col as f32 * TILE_SIZE as f32 + offset.x,
            //                 y: row as f32 * TILE_SIZE as f32 + offset.y,
            //                 z: layer.z_index as f32,
            //             },
            //             ..default()
            //         },
            //     ));
            // }
        }
    }
}
