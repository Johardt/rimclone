use super::board::BoardInfo;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Component)]
#[require(TilePos, Sprite)]
pub struct Pawn;

pub fn spawn_pawn(mut commands: Commands, board_info: Res<BoardInfo>) {
    use rand::Rng;
    commands.spawn((
        Pawn,
        TilePos {
            x: rand::rng().random_range(0..board_info.board_size),
            y: rand::rng().random_range(0..board_info.board_size),
        },
        Sprite {
            ..Default::default()
        },
    ));
}
