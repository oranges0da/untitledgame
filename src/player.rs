use crate::animation::{PlayerAnimation, PlayerAnimationType, PlayerAnimations};
use crate::item::{PlayerItem, PlayerItems};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player {
    speed: f32, // movement speed of player on screen
    pub animation: PlayerAnimation,
    pub item: Option<PlayerItem>,
    pub frame_time: f32, // compare player frame_time to animation frame_time
}

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
            .add_systems(Update, player_jump);
    }
}

fn spawn_player(
    mut commands: Commands,
    animation_res: Res<PlayerAnimations>,
    item_res: Res<PlayerItems>,
) {
    // get current animation from global animation resource to use in Player component
    let Some(animation) = animation_res.get(PlayerAnimationType::Idle) else {
        error!("Failed to find animation: Idle");
        return;
    };

    let Some(item) = item_res.get("ice_cream".to_string()) else {
        return;
    };

    commands
        .spawn((
            SpriteSheetBundle {
                transform: Transform::from_scale(Vec3::new(2.2, 2.2, 0.))
                    .with_translation(Vec3::new(0., 0., 1.)), // z field of translation vector will determine z-index (overlay player over background)
                ..default()
            },
            Player {
                speed: 200.,
                animation,
                item: Some(item),
                frame_time: 0.6,
            },
        ))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(AdditionalMassProperties::Mass(10.0)) // set mass of player
        .insert(GravityScale(2.0)) // subject player to gravity
        .insert(Collider::cuboid(12., 16.));
}

fn move_player(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((player, mut player_pos)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left])
            && player_pos.translation.x < 480.
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
