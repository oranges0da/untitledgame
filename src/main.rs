use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_rapier2d::prelude::*;

mod animation;
mod globals;
mod map;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // necessary to not spawn blurry sprites
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Untitled Game".into(),
                        resolution: (globals::HEIGHT, globals::WIDTH).into(),
                        present_mode: PresentMode::AutoVsync,
                        // Tells wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, follow_player)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(player::PlayerPlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(map::MapPlugin)
        .run();
}

#[derive(Component)]
struct PlayerCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera));
}

// camera follows player (player is always centered on screen)
fn follow_player(
    player_query: Query<&Transform, With<player::Player>>,
    mut camera: Query<(&PlayerCamera, &mut Transform), Without<player::Player>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    let Ok((_, mut camera_transform)) = camera.get_single_mut() else {
        return;
    };

    // set camera's coordinates to player's coordinates on each frame
    camera_transform.translation = player.translation;
}
