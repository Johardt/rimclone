use crate::math::matrix::Matrix;
use bevy::prelude::*;

pub const BOARD_SIZE: usize = 8;
pub const TILE_SIZE: usize = 32;

#[derive(Component, Clone)]
#[require(Position, WalkSpeed)]
pub struct Tile;

#[derive(Component, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Used for pathfinding and collision.
/// Walking speed of 0.0 means not walkable.
#[derive(Component, Default)]
pub struct WalkSpeed(pub f32);

pub struct BoardLayer {
    pub tiles: Matrix<Option<Tile>>,
}

impl BoardLayer {
    pub fn new() -> Self {
        let mut tiles = Matrix::new(BOARD_SIZE, BOARD_SIZE);

       tiles.iter_mut().for_each(|tile| {
           if rand::random::<f32>() < 0.7 {
               *tile = Some(Tile)
           }
       }); 

        BoardLayer { tiles }
    }
}
