use bevy::prelude::*;

use crate::animation;

const PLAYER_SIZE: f32 = 3.; // factor to enlarge the player sprite
const PLAYER_SPEED: f32 = 250.; // factor to multiply translation

const JUMP_POWER: f32 = 2.;
const FALL_POWER: f32 = 4.;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(player_jump)
            .add_system(player_fall)
            .add_system(ground_detection);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    animation_res: Res<animation::PlayerAnimations>,
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
    let Some(animation) = animation_res.get(animation::Animation::Idle) else {error!("Failed to find animation: Idle"); return;};

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
        animation,
        animation::FrameTime(0.),
    ));
}

fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // getting mutable player_transform property for every SINGLE frame (get_single_mut makes sense now.)
    if let Ok(mut player_pos) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1., 0., 0.);
        }

        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
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

fn player_jump(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
            player_transform.translation.y += 1. * JUMP_POWER;
        } else {
            player_transform.translation.y -= 1.;
        }
    }
}

fn player_fall(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
            player_transform.translation.y -= 1. * FALL_POWER;
        }
    }
}

fn ground_detection(mut player_query: Query<&mut Transform, With<Player>>) {
    let Ok(mut player_transform) = player_query.get_single_mut() else {return;};

    if player_transform.translation.y < -50. {
        player_transform.translation.y = -50.;
    }
}
