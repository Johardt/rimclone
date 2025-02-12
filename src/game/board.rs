use crate::math::matrix::Matrix;
use bevy::prelude::*;

pub const BOARD_SIZE: usize = 128;
pub const TILE_SIZE: usize = 32;

#[derive(Component, Clone)]
pub struct Tile {
    id: u32,
}

#[derive(Resource)]
pub struct BoardLayer {
    pub tiles: Matrix<Option<Tile>>,
}

impl BoardLayer {
    pub fn new() -> Self {
        let mut tiles = Matrix::new(BOARD_SIZE, BOARD_SIZE);

       tiles.iter_mut().for_each(|tile| {
           if rand::random::<f32>() < 0.7 {
               *tile = Some(Tile {id: 0})
           }
       }); 

        BoardLayer { tiles }
    }
}
