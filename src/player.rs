use crate::animation::{Direction, PlayerAnimationType};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub animation: PlayerAnimationType,
    pub direction: Direction,
    pub frame_time: f32, // To compare player's frame_time to animation's frame_time.
}

impl Player {
}

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, set_player_direction);
    }
}

fn spawn_player(mut commands: Commands) {
    const SCALE: f32 = 0.75;

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                scale: Vec3::new(SCALE, SCALE, 0.),
                translation: Vec3::new(0., 0., 1.), // Setting z-index to 1 will make sure player is drawn over everything else.
                ..default()
            },
            ..default()
        },
        Player {
            animation: PlayerAnimationType::Idle(Direction::South),
            direction: Direction::South,
            frame_time: 0.6,
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
    }

    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        direction += Vec3::new(1., 0., 0.);
    }

    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        direction += Vec3::new(0., 1., 0.);
    }

    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        direction += Vec3::new(0., -1., 0.);
    }

    if direction.length() > 0. {
        direction = direction.normalize(); // Normalizing direction for diagonal movement.
    }

    // Setting translation vector to product of updated direction vector
    // delta_seconds returns time elapsed since last frame, used to make movement frame-rate independent
    pos.translation += direction * SPEED * time.delta_seconds();
}

fn set_player_direction(mut player_q: Query<&mut Player>, keyboard_input: Res<Input<KeyCode>>) {
    let mut player = player_q.single_mut();

    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])
    {
        player.direction = Direction::NorthWest;
    } else if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])
    {
        player.direction = Direction::NorthEast;
    } else if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_pressed([KeyCode::S, KeyCode::Down])
    {
        player.direction = Direction::SouthWest;
    } else if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_pressed([KeyCode::S, KeyCode::Down])
    {
        player.direction = Direction::SouthEast;
    } else if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        player.direction = Direction::North;
    } else if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        player.direction = Direction::South;
    } else if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        player.direction = Direction::West;
    } else if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        player.direction = Direction::East;
    }
}
