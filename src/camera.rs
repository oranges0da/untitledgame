use crate::player;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, player_camera);
    }
}

#[derive(Component)]
struct PlayerCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera));
}

// Camera follows player, always centered on screen.
fn player_camera(
    player_query: Query<&Transform, With<player::Player>>,
    mut camera: Query<(&PlayerCamera, &mut Transform), Without<player::Player>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    let Ok((_, mut camera_transform)) = camera.get_single_mut() else {
        return;
    };

    // Set camera's coordinates to player's coordinates on each frame.
    camera_transform.translation = player.translation;
}
