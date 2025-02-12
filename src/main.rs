use bevy::prelude::*;
use game::{
    board::{BoardLayer, Position, Tile, WalkSpeed, BOARD_SIZE, TILE_SIZE},
    camera::CameraPlugin,
};

mod game;
mod math;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, query_tile)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_handle: Handle<Image> = asset_server.load("graphics/tile.png");

    let board = BoardLayer::new();

    let board_width = BOARD_SIZE * TILE_SIZE;
    let board_height = BOARD_SIZE * TILE_SIZE;
    let x_offset = -(board_width as f32) / 2.0 + TILE_SIZE as f32 / 2.0;
    let y_offset = -(board_height as f32) / 2.0 + TILE_SIZE as f32 / 2.0;

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if let Some(ref _tile) = board.tiles.get(row, col).unwrap() {
                commands.spawn((
                    Tile,
                    Position {x: col, y: row},
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

fn query_tile(mut query: Query<(&Position, &mut Sprite), With<Tile>>, input: Res<ButtonInput<KeyCode>>) {
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
