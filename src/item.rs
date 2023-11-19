use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {}
}

enum ItemType {
    Weapon,
    Food,
}

#[derive(Component)]
pub struct PlayerItem {
    name: String,
    itemType: ItemType,
    icon_path: String,
    index: i32,
}

#[derive(Resource)]
pub struct PlayerItems {
    items: HashMap<String, PlayerItem>,
}
