use bevy::prelude::*;
use std::collections::HashMap;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerItems>()
            .add_systems(Update, show_item_ui)
            .add_systems(Startup, spawn_idle_item);
    }
}

#[derive(Component, Clone)]
pub struct Item {
    pub position: Vec3,
    pub current_item: Option<PlayerItem>, // None if player has no items.
}

impl Item {
    pub fn is_holding_item(&self) -> bool {
        self.current_item.is_some()
    }

    pub fn get_current_item(&self) -> Option<&PlayerItem> {
        self.current_item.as_ref()
    }
}

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

// Show current item in corner of screen with nice ui. (Only if player is holding an item of course.)
fn show_item_ui(mut commands: Commands, asset_server: Res<AssetServer>, item: Query<&Item>) {
    if let Ok(item) = item.get_single() {
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
                        .load(item.current_item.clone().unwrap().icon_path)
                        .into(),
                    transform: Transform::from_scale(Vec3::new(2., 2., 0.)),
                    ..default()
                });
            });
    }
}

// Spawn item that player can pickup.
fn spawn_idle_item(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    item_res: Res<PlayerItems>,
) {
    let current_item = item_res.get("ice_cream".to_string()).unwrap();
    let item = Item {
        position: Vec3::new(-20., -60., 0.),
        current_item: item_res.get("ice_cream".to_string()),
    };

    commands.spawn(SpriteBundle {
        texture: asset_server.load(current_item.clone().icon_path),
        transform: Transform::from_translation(item.position),
        ..default()
    });
}
