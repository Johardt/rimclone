use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::text::FontSmoothing;
use game::common::Position;
use game::{
    board::{Board, BoardPlugin, BOARD_SIZE},
    camera::CameraPlugin,
    tile::Tile,
};
use rand::Rng;

mod game;
mod math;

fn main() {
    App::new()
        .insert_resource(Board::default())
        .add_plugins(FpsOverlayPlugin {
                        config: FpsOverlayConfig {
                            text_config: TextFont {
                                font_size: 42.0,
                                font: default(),
                                font_smoothing: FontSmoothing::default(),
                            },
                            text_color: Color::srgb(0.0, 1.0, 0.0),
                            enabled: true,
                        },
                    },
)
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
        let x = rand::rng().random_range(0..BOARD_SIZE);
        let y = rand::rng().random_range(0..BOARD_SIZE);
        for (pos, mut sprite) in query.iter_mut() {
            if pos.x == x && pos.y == y {
                sprite.color = Color::srgb(0.0, 1.0, 0.0);
            }
        }
    }
}
