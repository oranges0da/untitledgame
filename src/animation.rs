use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimations>()
            .add_systems(Update, animate_player)
            .add_systems(Update, change_player_animation)
            .add_systems(Update, flip_sprite);
    }
}

#[derive(Component, Clone, Debug)]
pub struct SpriteAnimation {
    pub len: usize,
    pub frame_time: f32,
    pub path: String,
}
#[derive(Component, Debug)]
pub struct FrameTime(pub f32);

#[derive(Component, Eq, PartialEq, Hash, Debug)]
pub enum Animation {
    Idle,
    Run,
    Jump,
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
                len: 5,
                frame_time: 0.2,
                path: "player/idle.png".to_string(),
            },
        );
        map.add(
            Animation::Run,
            SpriteAnimation {
                len: 5,
                frame_time: 0.12,
                path: "player/run.png".to_string(),
            },
        );

        map.add(
            Animation::Jump,
            SpriteAnimation {
                len: 1,
                frame_time: 0.,
                path: "player/jump.png".to_string(),
            },
        );

        map
    }
}

// fundamental animation logic, will be the same for any animation implemented
fn animate_player(
    mut player_query: Query<(&mut Player, &mut TextureAtlasSprite), With<Player>>, // cannot include TextureAtlasSprite in Player{} due to the way Bevy renders entities
    time: Res<Time>,
) {
    for (mut player, mut sprite) in player_query.iter_mut() {
        // get time elapsed (f32) since last frame
        player.frame_time += time.delta_seconds();

        if player.frame_time > player.animation.frame_time {
            let frames_elapsed = (player.frame_time / player.animation.frame_time) as usize;
            sprite.index += frames_elapsed;

            // if sprite index becomes greater than length of total animation frames, reset sprite index
            if sprite.index >= player.animation.len {
                sprite.index %= player.animation.len;
            }

            // subtract total frames from frame_time as to not accumulate in frame_time
            player.frame_time -= player.animation.frame_time * frames_elapsed as f32;
        }
    }
}

// change player animation and texture_atlas (spritesheet) according to action
fn change_player_animation(
    mut player: Query<&mut Player>,
    player_transform_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    animations: Res<PlayerAnimations>,
    mut texture_atlas_query: Query<&mut Handle<TextureAtlas>, With<Player>>,
) {
    let mut player = player.single_mut();
    let player_transform = player_transform_query.single();
    let mut atlas = texture_atlas_query.single_mut();

    let path = if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right, KeyCode::A, KeyCode::Left])
    // to not play running animation when pressing jump and left or right at same time
    && !keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) && player_transform.translation.y <= 0.
    {
        "player/run.png"
    } else if keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space]) {
        "player/jump.png"
    } else if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        "player/jump.png"
    } else if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])
        && keyboard_input.any_pressed([KeyCode::A, KeyCode::D, KeyCode::Left, KeyCode::Right])
    {
        "player/jump.png"
    } else {
        "player/idle.png"
    };

    // load spritesheet and split into grid of individual sprites and convert to spritesheet handle
    let texture_handle = asset_server.load(path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(30., 33.),
        5,
        1,
        Some(Vec2::new(2., 0.)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let curr_animation = if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right, KeyCode::A, KeyCode::Left])
            // to not play running animation when pressing jump and left or right at same time
            && !keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) && player_transform.translation.y <= 0.
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
    } else {
        Animation::Idle
    };

    // get SpriteAnimation data from Animation enum and set accordingly (this is very jerry-rigged for now.)
    let Some(new_animation) = animations.get(curr_animation) else {
        return ();
    };

    player.animation = new_animation;
    *atlas = texture_atlas_handle;
}

fn flip_sprite(
    mut player_sprite: Query<&mut TextureAtlasSprite, With<Player>>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
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
}
