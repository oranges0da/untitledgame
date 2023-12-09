use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerItems>()
            .add_systems(Startup, spawn_item)
            .add_systems(Update, show_item_ui);
    }
}

#[derive(Component)]
pub struct Item;

#[derive(Clone, Debug)]
enum ItemType {
    Food,
}

#[derive(Component, Clone)]
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
        return self.items.get(&id).cloned();
    }
}

// init items for global resource
impl FromWorld for PlayerItems {
    fn from_world(_world: &mut World) -> Self {
        let mut items = PlayerItems {
            items: HashMap::new(),
        };

        items.add(
            "ice_cream".to_string(),
            PlayerItem {
                name: "Ice Cream".to_string(),
                item_type: ItemType::Food,
                icon_path: "item/food/ice_cream.png".to_string(),
                index: 0,
            },
        );

        items
    }
}

// show current item in corner of screen
fn show_item_ui(mut commands: Commands, asset_server: Res<AssetServer>, player: Query<&Player>) {
    let player = player.single();

    if player.item.is_some() {
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
                    image: asset_server
                        .load(player.item.clone().unwrap().icon_path)
                        .into(),
                    transform: Transform::from_scale(Vec3::new(2., 2., 0.)),
                    ..default()
                });
            });
    }
}

// Spawn item in player's hands.
fn spawn_item(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("item/food/ice_cream.png"),
            transform: Transform {
                translation: Vec3::new(10., 10., 10.),
                ..default()
            },
            ..default()
        })
        .insert(Item);
}
