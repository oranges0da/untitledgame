use crate::animation::{PlayerAnimation, PlayerAnimationType, PlayerAnimations};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player {
    speed: f32,
    pub animation: PlayerAnimation,
    pub frame_time: f32, // To compare player's frame_time to animation's frame_time.
}

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, player_jump);
    }
}

fn spawn_player(mut commands: Commands, animation_res: Res<PlayerAnimations>) {
    // Get idle animation to play on spawn.
    let Some(animation) = animation_res.get(PlayerAnimationType::Idle) else {
        error!("Failed to find animation: Idle");
        return;
    };

    commands
        .spawn((
            SpriteSheetBundle {
                transform: Transform {
                    scale: Vec3::new(2.2, 2.2, 0.),
                    translation: Vec3::new(0., 0., 1.), // Setting z-index to 1 will make sure player is drawn over everything else.
                    ..default()
                },
                ..default()
            },
            Player {
                speed: 200.,
                animation,
                frame_time: 0.6,
            },
        ))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(AdditionalMassProperties::Mass(10.0)) // Set mass of player.
        .insert(GravityScale(2.0)) // Subject player body to gravity.
        .insert(Collider::cuboid(10., 16.));
}

fn player_movement(
    mut player_q: Query<(&Player, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((player, mut pos)) = player_q.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
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
        pos.translation += direction * player.speed * time.delta_seconds();
    } else {
        info!("Could not parse player_transform when moving player.");
    }
}

fn player_jump(
    mut vel_q: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut vel = vel_q.single_mut();

    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::Space, KeyCode::W]) {
        vel.linvel.y = 100.;
    } else {
        vel.linvel.y = vel.linvel.y.min(0.0);
    }
}