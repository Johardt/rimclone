use super::common::Position;
use bevy::prelude::*;

pub const TILE_SIZE: usize = 32;

#[derive(Component, Clone)]
#[require(Position, WalkSpeed)]
pub struct Tile;

impl Default for Tile {
    fn default() -> Self {
        Tile
    }
}

/// Used for calculating cost in pathfinding
#[derive(Component, Default)]
pub struct WalkSpeed(pub f32);
