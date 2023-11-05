use bevy::prelude::*;
use bevy::window::PresentMode;

mod animation;
mod globals;
mod player;

pub const HEIGHT: f32 = 640.;
pub const WIDTH: f32 = 480.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // necessary to not spawn blurry sprites
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Popcorn Guy".into(),
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
        .add_plugins(player::PlayerPlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
