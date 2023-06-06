use bevy::prelude::*;

pub const PLAYER_SIZE: f32 = 4.; // factor to enlarge the player sprite

#[derive(Component)]
pub struct Player {}

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
    // load spritesheet and split into grid of individual sprites
    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(30., 16.),
        4,
        5,
        Some(Vec2::new(0., 0.)),
        Some(Vec2::new(0., 0.)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 4, // index of which sprite to spawn in sheet
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(PLAYER_SIZE, PLAYER_SIZE, 0.)), // make sprite bigger by a factor of PLAYER_SIZE
            ..default()
        },
        Player {},
    ));
}
