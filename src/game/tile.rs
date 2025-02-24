use bevy::prelude::*;
use bevy_ecs_tilemap::{map::TilemapId, tiles::*};

pub const TILE_SIZE: usize = 32;

/// Will eventually use the bevy_ecs_tilemap Tile, but that is still a bundle:
/// https://github.com/StarArawn/bevy_ecs_tilemap/blob/56dc41b408bd6137f3b92d7c44715174414cc489/src/tiles/mod.rs#L114
#[derive(Component, Clone)]
#[require(TilePos, TileTextureIndex, TilemapId, TileVisible, TileFlip, TileColor)]
pub struct Tile;

impl Default for Tile {
    fn default() -> Self {
        Tile
    }
}
