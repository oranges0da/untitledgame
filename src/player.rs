use crate::animation;
use crate::globals;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    speed: f32, // movement speed of player on screen
    jump_speed: f32,
    fall_speed: f32, // how quickly player falls
    pub animation: animation::SpriteAnimation,
    pub frame_time: f32, // compare player frame_time to animation frame_time
}

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
            .add_systems(Update, player_jump)
            .add_systems(Update, player_fall)
            .add_systems(Update, ground_detection);
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
        Vec2::new(30., 32.),
        9,
        8,
        Some(Vec2::new(2., 0.5)),
        Some(Vec2::new(0., 0.)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let Some(animation) = animation_res.get(animation::Animation::Idle) else {
        error!("Failed to find animation: Idle");
        return;
    };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0, // index of which sprite to spawn in sheet
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(2., 2., 0.)), // make sprite bigger by a factor of PLAYER_SIZE
            ..default()
        },
        Player {
            speed: globals::SPEED,
            jump_speed: globals::JUMP_SPEED,
            fall_speed: globals::FALL_SPEED,
            animation,
            frame_time: 0.6,
        },
        Jump(100.),
    ));
}

fn move_player(
    mut player_query: Query<(&Player, &mut Transform), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((player, mut player_pos)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left])
            && player_pos.translation.x < globals::WIDTH
        {
            direction += Vec3::new(-1., 0., 0.);
        }

        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            direction += Vec3::new(1., 0., 0.);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize(); // allows sprite to move diagonally
        }

        // setting translation vector to our own updated direction vector
        // delta_seconds returns time elapsed since last frame, used to make movement frame-rate independent
        player_pos.translation += direction * player.speed * time.delta_seconds();
    } else {
        info!("Could not parse player_transform");
    }
}

#[derive(Component)]
struct Jump(f32);

fn player_jump(
    mut player: Query<(&Player, &mut Transform, &mut Jump), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((player, mut player_transform, mut jump)) = player.get_single_mut() else {
        return;
    };

    // acceleration
    let jump_power: f32 = time.delta_seconds() * player.jump_speed * 2.;

    jump.0 -= jump_power;

    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Space, KeyCode::Up])
        && player_transform.translation.y < globals::CEILING
    {
        player_transform.translation.y += jump_power;
    }
}

fn player_fall(
    mut player_query: Query<(&Player, &mut Transform), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((player, mut player_transform)) = player_query.get_single_mut() {
        let fall_power: f32 = player.fall_speed * time.delta_seconds() * 2.;

        if !keyboard_input.any_pressed([KeyCode::Up, KeyCode::W, KeyCode::Space])
            && player_transform.translation.y < globals::CEILING * 2.
        {
            player_transform.translation.y -= fall_power;
        }
    }
}

fn ground_detection(mut player_query: Query<&mut Transform, With<Player>>) {
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        return;
    };

    if player_transform.translation.y < -globals::CEILING {
        player_transform.translation.y = -globals::CEILING;
    }
}
