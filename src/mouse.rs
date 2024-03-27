use bevy::prelude::*;
use bevy::window::*;

#[derive(Component)]
pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor)
            .add_systems(Update, update_cursor);
    }
}

#[derive(Component)]
struct Mouse;

fn spawn_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    const SCALE: f32 = 2.;

    let texture_handle = asset_server.load("ui/mouse.png");
    let texture = TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 4, 1, None, None);
    let texture_atlas = texture_atlases.add(texture);

    commands.spawn(SpriteSheetBundle {
        texture_atlas,
        transform: Transform::from_scale(Vec3::new(SCALE, SCALE, 0.)),
        ..default()
    })
    .insert(Mouse);
}

fn update_cursor(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mouse_entity_q: Query<Entity, With<Mouse>>,
    mut mouse_pos_q: Query<&mut Transform, With<Mouse>>,
) {
    let mouse_e = mouse_entity_q.single();
    let mut mouse_pos = mouse_pos_q.single_mut();

    if let Some(cursor_position) = window_q.single().cursor_position() {
       mouse_pos.translation.x = cursor_position.x;
       mouse_pos.translation.y = -cursor_position.y;
    } else {
        println!("Cursor is not in the game window.");
    }
}
