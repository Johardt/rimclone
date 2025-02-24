use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use game::{
    board::{BoardInfo, BoardPlugin},
    tile::Tile,
};
use helpers::camera::CameraPlugin;

mod game;
mod helpers;

fn hide_texture(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut TilemapTexture, &mut Visibility)>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyH) {
        for (_, mut visibility) in &mut query {
            *visibility = match *visibility {
                Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
            };
        }
    }
}

fn find_tiles(keyboard_input: Res<ButtonInput<KeyCode>>, query: Query<&TilePos, With<Tile>>) {
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        for tilepos in &query {
            println!("Tile found with position: {:?}", tilepos);
        }
    }
}

fn replace_tile(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&TilePos, &mut TileColor)>,
    board_info: Res<BoardInfo>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        let x = rand::random_range(0..board_info.board_size);
        let y = rand::random_range(0..board_info.board_size);

        for (tilepos, mut tilecolor) in &mut query {
            if tilepos.x == x && tilepos.y == y {
                tilecolor.0 = Color::srgb(rand::random(), rand::random(), rand::random());
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Basic Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_plugins((BoardPlugin, CameraPlugin))
        .add_systems(Update, (hide_texture, replace_tile, find_tiles))
        .run();
}
