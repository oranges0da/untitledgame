use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimations>()
            .add_system(animate_player)
            .add_system(change_animation);
    }
}

#[derive(Component, Clone)]
pub struct SpriteAnimation {
    pub starting_index: usize,
    pub len: usize,
    pub frame_time: f32,
}
#[derive(Component, Debug)]
pub struct FrameTime(pub f32);

#[derive(Component, Eq, PartialEq, Hash)]
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
                starting_index: 4,
                len: 5,
                frame_time: 0.5,
            },
        );
        map.add(
            Animation::Run,
            SpriteAnimation {
                starting_index: 8,
                len: 7,
                frame_time: 0.16,
            },
        );

        map.add(
            Animation::Jump,
            SpriteAnimation {
                starting_index: 1,
                len: 3,
                frame_time: 0.5,
            },
        );

        map.add(
            Animation::Fall,
            SpriteAnimation {
                starting_index: 3,
                len: 1,
                frame_time: 1.,
            },
        );

        map
    }
}

// fundamental animation logic, will be the same for any animation implemented
fn animate_player(
    mut animation: Query<(
        &mut TextureAtlasSprite,
        &mut SpriteAnimation,
        &mut FrameTime,
    )>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in animation.iter_mut() {
        // get time elapsed (f32) since last frame
        frame_time.0 += time.delta_seconds();

        if frame_time.0 > animation.frame_time {
            let frames_elapsed = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames_elapsed;

            // if sprite index becomes greater than length of total animation frames, reset sprite index
            if sprite.index - animation.starting_index >= animation.len {
                sprite.index %= animation.len;
                sprite.index += animation.starting_index;
            }

            // subtract total frames from frame_time as to not accumulate in frame_time
            frame_time.0 -= animation.frame_time * frames_elapsed as f32;
        }
    }
}

// change global PlayerAnimation resource to desired SpriteAnimation
fn change_animation(
    mut player: Query<
        (
            &mut TextureAtlasSprite,
            &mut SpriteAnimation,
            &mut FrameTime,
        ),
        With<Player>,
    >,
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
) {
    let (mut sprite, mut sprite_animation, mut _frame_time) = player.single_mut();
    let mut player_transform = player_query.single_mut();

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

    let curr_animation =
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right, KeyCode::A, KeyCode::Left])
            && !keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])
        // to not play running animation when pressing jump and left or right at same time
        {
            Animation::Run
        } else if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            Animation::Jump
        } else if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            Animation::Fall
        } else {
            Animation::Idle
        };

    // get SpriteAnimation data from Animation enum and set accordingly (this is very jerry-rigged for now.)
    let Some(new_animation) = animations.get(curr_animation) else {return ();};
    *sprite_animation = new_animation;
}
