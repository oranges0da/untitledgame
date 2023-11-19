use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerItems>()
            .add_systems(Update, show_item);
    }
}

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
    amount: i32,
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
                amount: 1,
            },
        );

        items
    }
}

// test system to make sure we can get specific item out of item_res
fn show_item(mut commands: Commands, item_res: Res<PlayerItems>, asset_server: Res<AssetServer>) {
    let Some(item) = item_res.get("peanut_butter".to_string()) else {
        return;
    };

    commands.spawn(SpriteBundle {
        texture: asset_server.load(item.icon_path),
        transform: Transform::from_scale(Vec3::new(3., 3., 0.)),
        ..default()
    });
}
