use crate::animation::{PlayerAnimation, PlayerAnimationType, PlayerAnimations, Direction};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub animation: PlayerAnimation,
    pub frame_time: f32, // To compare player's frame_time to animation's frame_time.
    pub is_facing_right: bool,
}

impl Player {
    pub fn is_facing_right(&self) -> bool {
        self.is_facing_right
    }
}

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(mut commands: Commands, animation_res: Res<PlayerAnimations>) {
    const SCALE: f32 = 3.;

    // Get idle animation to play on spawn.
    let Some(idle_animation) = animation_res.get(PlayerAnimationType::Idle(Direction::South)) else {
        error!("Failed to find animation: Idle");
        return;
    };

    commands
        .spawn((
            SpriteSheetBundle {
                transform: Transform {
                    scale: Vec3::new(SCALE, SCALE, 0.),
                    translation: Vec3::new(0., 0., 1.), // Setting z-index to 1 will make sure player is drawn over everything else.
                    rotation: Quat::IDENTITY, // Set the initial rotation to identity. (None, I think?)
                    ..default()
                },
                ..default()
            },
            Player {
                animation: idle_animation,
                frame_time: 0.6,
                is_facing_right: true, // Sprite is facing right.
            },
        ));
}

fn player_movement(
    mut player_q: Query<(&mut Player, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 250.;

    let (mut player, mut pos) = player_q.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        direction += Vec3::new(-1., 0., 0.);
        player.is_facing_right = false;
    }

    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        direction += Vec3::new(1., 0., 0.);
        player.is_facing_right = true;
    }

    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        direction += Vec3::new(0., 1., 0.);
    }

    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        direction += Vec3::new(0., -1., 0.);
    }

    if direction.length() > 0. {
        direction = direction.normalize(); // allows sprite to move diagonally
    }

    // Setting translation vector to product of updated direction vector
    // delta_seconds returns time elapsed since last frame, used to make movement frame-rate independent
    pos.translation += direction * SPEED * time.delta_seconds();
}
