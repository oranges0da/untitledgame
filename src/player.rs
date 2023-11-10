use crate::animation;
use crate::globals;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player {
    speed: f32,      // movement speed of player on screen
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
    // load spritesheet and split into grid of individual sprites and convert to spritesheet handle
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

    // get current animation from global animation resource to use in Player component
    let Some(animation) = animation_res.get(animation::Animation::Idle) else {
        error!("Failed to find animation: Idle");
        return;
    };

    commands
        .spawn((
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
                fall_speed: globals::FALL_SPEED,
                animation,
                frame_time: 0.6,
            },
        ))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default());
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
            direction += Vec3::new(-100., 0., 0.);
        }

        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            direction += Vec3::new(1., 0., 0.);
        }

        if direction.length() > 0. {
            direction = direction.normalize(); // allows sprite to move diagonally
        }

        // Setting translation vector to product of updated direction vector
        // delta_seconds returns time elapsed since last frame, used to make movement frame-rate independent
        // as well as player.speed stems from globals
        player_pos.translation += direction * player.speed * time.delta_seconds();
    } else {
        info!("Could not parse player_transform when moving player.");
    }
}

fn player_jump(
    mut velocity: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok(mut vel) = velocity.get_single_mut() else {
        panic!("Could not parse player query in player_jump.");
    };

    if keyboard_input.pressed(KeyCode::Up) {
        vel.linvel.y = 100.;
    } else {
        vel.linvel.y = vel.linvel.y.min(0.0);
    }
}

fn player_fall(
    mut player: Query<(&Transform, &mut Velocity), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok((pos, mut vel)) = player.get_single_mut() else {
        panic!("Could not parse player in player_fall.");
    };

    if pos.translation.y > 0. && !keyboard_input.pressed(KeyCode::Up) {
        vel.linvel.y = -50.
    }
}

fn ground_detection(mut player_query: Query<&mut Transform, With<Player>>) {
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        return;
    };

    if player_transform.translation.y < 0. {
        player_transform.translation.y = 0.
    }
}
