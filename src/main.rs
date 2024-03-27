use bevy::prelude::*;
use bevy::window::*;

mod animation;
mod camera;
mod debug;
mod item;
mod map;
mod mouse;
mod player;

fn main() {
    let mut app = App::new();

    app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // Necessary to not spawn blurry sprites.
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Untitled Game".into(),
                        resolution: (640., 480.).into(),
                        present_mode: PresentMode::AutoVsync,
                        // Tells wasm to resize the window according to the available canvas.
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        cursor: Cursor {
                            visible: false,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
    );
    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.))); // Set background color to black.
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(debug::DebugPlugin);
    app.add_plugins(map::MapPlugin);
    app.add_plugins(mouse::MousePlugin);
    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(animation::AnimationPlugin);

    app.run();
}
