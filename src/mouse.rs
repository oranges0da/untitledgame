use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_pos);
    }
}

fn mouse_pos(q_window: Query<&Window, With<PrimaryWindow>>) {
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position() {
        info!("Mouse X: {}, Mouse Y: {}", world_position.x, world_position.y);
    }
}
