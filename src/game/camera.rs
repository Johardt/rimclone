use bevy::{input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll}, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, camera_control_system);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
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