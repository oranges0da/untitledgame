use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameRate>()
            .add_plugins(LogDiagnosticsPlugin::default())
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Update, draw_average_fps);
    }
}

#[derive(Resource, Default)]
pub struct FrameRate(pub f64);

// Draw current FPS to corner of screen.
fn draw_average_fps(
    mut commands: Commands,
    diagnostics: Res<DiagnosticsStore>,
    mut frame_rate: ResMut<FrameRate>,
) {
    // Get average FPS from Bevy's diagnostics store.
    if let Some(fps) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
    {
        frame_rate.0 = fps
    } else {
        info!("Could not set frame_rate resource in main.");
    }

    commands.spawn(TextBundle {
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
    });
}
