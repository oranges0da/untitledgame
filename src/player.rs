use bevy::prelude::*;

const PLAYER_SIZE: f32 = 3.; // factor to enlarge the player sprite
const PLAYER_SPEED: f32 = 100.;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_system(move_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // load spritesheet and split into grid of individual sprites
    let texture_handle = asset_server.load("player_idle.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(25., 15.), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0, // index of which sprite to spawn in sheet
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(PLAYER_SIZE, PLAYER_SIZE, 0.)), // make sprite bigger by a factor of PLAYER_SIZE
            ..default()
        },
        Player {},
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    // getting mutable player_transform property for every SINGLE frame (get_single_mut makes sense now.)
    if let Ok(mut player_pos) = player_transform.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            info!("Up is pressed");
            direction += Vec3::new(0., 1., 0.);
        }

        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            info!("Up is pressed");
            direction += Vec3::new(0., -1., 0.);
        }

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            info!("Up is pressed");
            direction += Vec3::new(-1., 0., 0.);
        }

        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            info!("Up is pressed");
            direction += Vec3::new(1., 0., 0.);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize(); // allows sprite to move diagonally
        }

        // setting translation property to our own updated direction vector
        // delta_seconds returns time elapsed since last frame, used to make movement frame-rate independent
        player_pos.translation += direction * PLAYER_SPEED * time.delta_seconds();
    } else {
        info!("Could not parse player_transform");
    }
}
