use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimations>()
            .add_system(animate_player)
            .add_system(change_player_animation);
    }
}

#[derive(Component, Clone, Debug)]
pub struct SpriteAnimation {
    pub starting_index: usize,
    pub len: usize,
    pub frame_time: f32,
}
#[derive(Component, Debug)]
pub struct FrameTime(pub f32);

#[derive(Component, Eq, PartialEq, Hash, Debug)]
pub enum Animation {
    Idle,
    Run,
    Jump,
    Fall,
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub map: HashMap<Animation, SpriteAnimation>,
}

impl PlayerAnimations {
    pub fn add(&mut self, id: Animation, animation: SpriteAnimation) {
        self.map.insert(id, animation);
    }

    pub fn get(&self, id: Animation) -> Option<SpriteAnimation> {
        self.map.get(&id).cloned()
    }
}

// init PlayerAnimations resource with from_world and add animation data
impl FromWorld for PlayerAnimations {
    fn from_world(_world: &mut World) -> Self {
        let mut map = PlayerAnimations {
            map: HashMap::new(),
        };

        map.add(
            Animation::Idle,
            SpriteAnimation {
                starting_index: 0,
                len: 2,
                frame_time: 0.4,
            },
        );
        map.add(
            Animation::Run,
            SpriteAnimation {
                starting_index: 27,
                len: 8,
                frame_time: 0.12,
            },
        );

        map.add(
            Animation::Jump,
            SpriteAnimation {
                starting_index: 48,
                len: 2,
                frame_time: 0.5,
            },
        );

        map.add(
            Animation::Fall,
            SpriteAnimation {
                starting_index: 50,
                len: 1,
                frame_time: 1.,
            },
        );

        map
    }
}

// fundamental animation logic, will be the same for any animation implemented
fn animate_player(
    mut player_query: Query<(&mut Player, &mut TextureAtlasSprite)>, // cannot include TextureAtlasSprite in Player{} due to the way Bevy renders entities
    time: Res<Time>,
) {
    for (mut player, mut sprite) in player_query.iter_mut() {
        // get time elapsed (f32) since last frame
        player.frame_time += time.delta_seconds();

        if player.frame_time > player.animation.frame_time {
            let frames_elapsed = (player.frame_time / player.animation.frame_time) as usize;
            sprite.index += frames_elapsed;

            // if sprite index becomes greater than length of total animation frames, reset sprite index
            if sprite.index - player.animation.starting_index >= player.animation.len {
                sprite.index %= player.animation.len;
                sprite.index += player.animation.starting_index;
            }

            // subtract total frames from frame_time as to not accumulate in frame_time
            player.frame_time -= player.animation.frame_time * frames_elapsed as f32;
        }
    }
}

// change animation component attached Player entity to desired SpriteAnimation
fn change_player_animation(
    mut player: Query<&mut Player>,
    mut player_sprite: Query<&mut TextureAtlasSprite, With<Player>>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
) {
    let mut player = player.single_mut();
    let mut sprite = player_sprite.single_mut();
    let mut player_transform = player_transform_query.single_mut();

    // flip sprite on x axis when going from left to right, or vice-verse
    if keyboard_input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = true;
        player_transform.translation.x += 35.; // offset x by player width
    } else if keyboard_input.any_just_pressed([KeyCode::D, KeyCode::Right])
        && !keyboard_input.any_pressed([KeyCode::A, KeyCode::Left])
    {
        sprite.flip_x = false;
        player_transform.translation.x -= 35.;
    } else if keyboard_input.any_just_released([KeyCode::A, KeyCode::Left])
        && !keyboard_input.any_pressed([KeyCode::A, KeyCode::Left])
        && keyboard_input.any_pressed([KeyCode::D, KeyCode::Right])
    {
        sprite.flip_x = false;
    }

    let curr_animation = if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right, KeyCode::A, KeyCode::Left])
            // to not play running animation when pressing jump and left or right at same time
            && !keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])
    {
        Animation::Run
    } else if keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space]) {
        Animation::Jump
    } else if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        Animation::Jump
    } else if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])
        && keyboard_input.any_pressed([KeyCode::A, KeyCode::D, KeyCode::Left, KeyCode::Right])
    {
        Animation::Jump
    } else if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        Animation::Fall
    } else if player_transform.translation.y > -50.
        && !keyboard_input.any_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space])
    {
        Animation::Fall
    } else {
        Animation::Idle
    };

    // get SpriteAnimation data from Animation enum and set accordingly (this is very jerry-rigged for now.)
    let Some(new_animation) = animations.get(curr_animation) else {return ();};
    player.animation = new_animation;
    info!("Current animation: {:?}", player.animation);
}
