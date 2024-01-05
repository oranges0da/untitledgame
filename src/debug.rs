use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use std::fmt::Write;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameRate>()
            .add_plugins(LogDiagnosticsPlugin::default())
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, spawn_fps_text)
            .add_systems(Update, update_fps)
            .add_systems(Update, update_fps_text);
    }
}

#[derive(Resource, Default)]
struct FrameRate(f64);

#[derive(Component)]
struct FrameRateText;

// Spawn FPS in corner of screen.
fn spawn_fps_text(mut commands: Commands, frame_rate: Res<FrameRate>) {
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                format!("FPS: {}", frame_rate.0),
                TextStyle {
                    color: Color::WHITE,
                    ..default()
                },
            ),
            style: Style {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                // Offset from top-left corner.
                top: Val::Px(10.),
                left: Val::Px(10.),
                ..default()
            },
            ..default()
        })
        .insert(FrameRateText);
}

// Update FPS to current average.
fn update_fps(mut frame_rate: ResMut<FrameRate>, diagnostics: Res<DiagnosticsStore>) {
    let Some(fps) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
    else {
        return;
    };

    frame_rate.0 = fps;
}

// Update FPS text on screen.
fn update_fps_text(
    mut text_query: Query<&mut Text, With<FrameRateText>>,
    frame_rate: Res<FrameRate>,
) {
    for mut text in text_query.iter_mut() {
        let value = &mut text.sections[0].value;
        value.clear();

        write!(value, "FPS: {:.0}", frame_rate.0).unwrap();
    }
}
