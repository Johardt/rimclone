use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};
use game::board::{BoardLayer, BOARD_SIZE, TILE_SIZE};

mod game;
mod math;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, camera_control_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
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

    commands.insert_resource(board);
}

fn camera_control_system(
    mut query: Query<&mut Transform, With<Camera2d>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let drag_sensitivity = 1.0;
        if mouse_button_input.pressed(MouseButton::Left) {
            transform.translation.x -= mouse_motion.delta.x * drag_sensitivity;
            transform.translation.y += mouse_motion.delta.y * drag_sensitivity;
        }
        
        if mouse_scroll.delta.y != 0.0 {
            let zoom_sensitivity = 0.01;
            let scroll_amount = mouse_scroll.delta.y;
            let scale_change = 1.0 - scroll_amount * zoom_sensitivity;
            let new_scale = (transform.scale.x * scale_change).clamp(0.5, 5.0);
            transform.scale = Vec3::splat(new_scale);
        }
    }
}
