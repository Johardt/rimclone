use bevy::prelude::*;
use game::{
    tile::{Tile, Position},
    board::{Board, BoardPlugin, BOARD_SIZE},
    camera::CameraPlugin,
};

mod game;
mod math;

fn main() {
    App::new()
        .insert_resource(Board::default())
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CameraPlugin)
        .add_plugins(BoardPlugin)
        .add_systems(Update, query_tile)
        .run();
}

fn query_tile(
    mut query: Query<(&Position, &mut Sprite), With<Tile>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Enter) {
        let x = rand::random::<u8>() as usize % BOARD_SIZE;
        let y = rand::random::<u8>() as usize % BOARD_SIZE;
        for (pos, mut sprite) in query.iter_mut() {
            if pos.x == x && pos.y == y {
                sprite.color = Color::srgb(0.0, 1.0, 0.0);
            }
        }
    }
}
