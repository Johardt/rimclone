use crate::math::matrix::Matrix;
use bevy::prelude::*;

pub const BOARD_SIZE: usize = 8;
pub const TILE_SIZE: usize = 32;

#[derive(Component, Clone)]
#[require(Position, WalkSpeed)]
pub struct Tile;

#[derive(Component, Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Used for pathfinding and collision.
/// Walking speed of 0.0 means not walkable.
#[derive(Component, Default)]
pub struct WalkSpeed(pub f32);

/// BoardLayer is an abstract representation of a layer of the game board.
/// It doesn't exist in game, but instead determines how the tiles will be
/// spawned during setup.
#[derive(Default, Clone)]
pub struct BoardLayer {
    pub tiles: Matrix<Option<Tile>>,
}

impl BoardLayer {
    pub fn from_random(seed: f32) -> Self {
        let mut tiles = Matrix::new(BOARD_SIZE, BOARD_SIZE);

        tiles.iter_mut().for_each(|tile| {
            //TODO actually use the seed
            if rand::random::<f32>() < seed {
                *tile = Some(Tile)
            }
        });

        BoardLayer { tiles }
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

    pub fn pop_layer(&mut self) -> Option<BoardLayer> {
        self.layers.pop()
    }
}

impl Default for Board {
    fn default() -> Self {
        let layer = BoardLayer::from_random(0.7);
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

fn board_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
    existing_tiles: Query<Entity, With<Tile>>,
) {
    for entity in existing_tiles.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let tile_handle: Handle<Image> = asset_server.load("graphics/tile.png");

    let board_width = BOARD_SIZE * TILE_SIZE;
    let board_height = BOARD_SIZE * TILE_SIZE;
    let x_offset = -(board_width as f32) / 2.0 + TILE_SIZE as f32 / 2.0;
    let y_offset = -(board_height as f32) / 2.0 + TILE_SIZE as f32 / 2.0;

    for layer in &board.layers {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(ref _tile) = layer.tiles.get(row, col).unwrap() {
                    commands.spawn((
                        Tile,
                        Position { x: col, y: row },
                        WalkSpeed(1.0),
                        Sprite {
                            image: tile_handle.clone(),
                            ..default()
                        },
                        Transform {
                            translation: Vec3 {
                                x: col as f32 * TILE_SIZE as f32 + x_offset,
                                y: row as f32 * TILE_SIZE as f32 + y_offset,
                                z: 0.0,
                            },
                            scale: Vec3::splat(1.0),
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}
