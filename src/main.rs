use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;

mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // necessary to not spawn blurry sprites
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Popcorn Guy".into(),
                        resolution: (640., 480.).into(),
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
        // show framerate in console for debug
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
