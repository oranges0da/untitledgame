use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerItems>()
            .add_systems(Startup, spawn_player_item)
            .add_systems(Update, animate_item)
            .add_systems(Update, show_item_ui);
    }
}

#[derive(Component)]
pub struct Item;

#[derive(Clone, Debug)]
enum ItemType {
    Weapon,
    Food,
}

#[derive(Component, Clone, Debug)]
pub struct PlayerItem {
    name: String,
    item_type: ItemType,
    icon_path: String,
    index: i32,
}

#[derive(Resource)]
pub struct PlayerItems {
    items: HashMap<String, PlayerItem>,
}

impl PlayerItems {
    pub fn add(&mut self, id: String, item: PlayerItem) {
        self.items.insert(id, item);
    }

    pub fn get(&self, id: String) -> Option<PlayerItem> {
        self.items.get(&id).cloned()
    }
}

// init items for global resource
impl FromWorld for PlayerItems {
    fn from_world(_world: &mut World) -> Self {
        let mut items = PlayerItems {
            items: HashMap::new(),
        };

        items.add(
            "peanut_butter".to_string(),
            PlayerItem {
                name: "Peanut Butter".to_string(),
                item_type: ItemType::Food,
                icon_path: "item/food/peanut_butter.png".to_string(),
                index: 0,
            },
        );

        items
    }
}

// show current item in corner of screen
fn show_item_ui(
    mut commands: Commands,
    item_res: Res<PlayerItems>,
    asset_server: Res<AssetServer>,
) {
    // get current item (pb) and load its image
    let Some(curr_item) = item_res.get("peanut_butter".to_string()) else {
        return;
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(30.),
                height: Val::Px(30.),
                top: Val::Px(30.),
                right: Val::Px(30.),
                border: UiRect::all(Val::Px(5.)),
                padding: UiRect::all(Val::Px(30.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            border_color: BorderColor(Color::WHITE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: asset_server.load(curr_item.clone().icon_path).into(),
                transform: Transform::from_scale(Vec3::new(2., 2., 0.)),
                ..default()
            });
        });
}

// spawn item in player's hands
fn spawn_player_item(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    item_res: Res<PlayerItems>,
) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("item/food/peanut_butter.png").into(),
            transform: Transform::from_translation(Vec3::new(100., 10., 2.))
                .with_scale(Vec3::new(0.8, 0.8, 0.)),
            ..default()
        })
        .insert(Item);
}

fn animate_item(
    player: Query<&Transform, With<Player>>,
    mut item: Query<(&Item, &mut Transform), Without<Player>>,
) {
    let player_pos = player.single();
    let Ok((_, mut item_pos)) = item.get_single_mut() else {
        return;
    };

    item_pos.translation.x = player_pos.translation.x + 25.;
    item_pos.translation.y = player_pos.translation.y - 15.;
}
