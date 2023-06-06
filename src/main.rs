use bevy::prelude::*;

mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // necessary to not spawn blurry sprites
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Popcorn Guy".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
