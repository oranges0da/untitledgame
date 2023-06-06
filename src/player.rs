use bevy::prelude::*;

pub const PLAYER_SIZE: f32 = 10.; // factor to enlarge player sprite

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(40., 15.), 4, 4, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite {
            index: 8, // which sprite to spawn
            ..default()
        },
        transform: Transform::from_scale(Vec3::splat(PLAYER_SIZE)),
        ..default()
    });
}
