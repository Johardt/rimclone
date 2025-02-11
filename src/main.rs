use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

const BOARD_SIZE: usize = 16;
const TILE_SIZE: usize = 32;

struct Tile {
    id: u32,
}

#[derive(Resource)]
struct BoardLayer {
    tiles: [[Option<Tile>; BOARD_SIZE]; BOARD_SIZE],
}

impl BoardLayer {
    fn new() -> Self {
        let mut tiles: [[Option<Tile>; BOARD_SIZE]; BOARD_SIZE] =
            core::array::from_fn(|_| core::array::from_fn(|_| None));

        (0..BOARD_SIZE).for_each(|i| {
            (0..BOARD_SIZE).for_each(|j| {
                if rand::random::<f32>() < 0.7 {
                    tiles[i][j] = Some(Tile {
                        id: (i * BOARD_SIZE + j) as u32,
                    });
                }
            });
        });

        BoardLayer { tiles }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_move_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let tile_handle: Handle<Image> = asset_server.load("tile.png");

    let board = BoardLayer::new();

    let board_width = BOARD_SIZE * TILE_SIZE;
    let board_height = BOARD_SIZE * TILE_SIZE;
    let x_offset = -(board_width as f32) / 2.0 + TILE_SIZE as f32 / 2.0;
    let y_offset = -(board_height as f32) / 2.0 + TILE_SIZE as f32 / 2.0;

    for row in 0..BOARD_SIZE {
        for column in 0..BOARD_SIZE {
            if let Some(ref _tile) = board.tiles[row][column] {
                commands.spawn((
                    Sprite {
                        image: tile_handle.clone(),
                        ..default()
                    },
                    Transform {
                        translation: Vec3 {
                            x: column as f32 * TILE_SIZE as f32 + x_offset,
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

fn mouse_move_system(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
) {
    if accumulated_mouse_motion.delta != Vec2::ZERO {
        let delta = accumulated_mouse_motion.delta;
        info!("mouse moved {}, {}", delta.x, delta.y);
    }

    if accumulated_mouse_scroll.delta != Vec2::ZERO {
        let delta = accumulated_mouse_scroll.delta;
        info!("mouse scrolled {}, {}", delta.x, delta.y);
    }
}
