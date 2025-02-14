use bevy::prelude::*;
use super::{board::BOARD_SIZE, common::Position};

#[derive(Component)]
#[require(Position)]
pub struct Pawn;

pub fn spawn_pawn(mut commands: Commands) {
    use rand::Rng;
    commands.spawn((
        Pawn,
        Position {
            x: rand::rng().random_range(0..BOARD_SIZE),
            y: rand::rng().random_range(0..BOARD_SIZE),
        }
    ));
}