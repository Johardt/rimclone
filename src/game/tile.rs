use bevy::prelude::*;

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