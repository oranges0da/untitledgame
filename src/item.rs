use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;
use crate::player::Player;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerItems>()
            .add_systems(Startup, spawn_idle_item)
            .add_systems(Update, show_item_ui)
            .add_systems(Update, item_pickup);
    }
}

#[derive(Component, Clone)]
pub struct Item {
    pub position: Vec3,
    pub current_item: Option<PlayerItem>, // None if player has no items.
    pub in_inv: bool, // Is player holding item?
}

impl Item {
    pub fn new(position: Vec3, current_item: Option<PlayerItem>, in_inv: bool) -> Self {
        Item {
            position,
            current_item,
            in_inv
        }
    }

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

// Show current item in corner of screen with nice ui.
fn show_item_ui(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    item_q: Query<&Item>,
) {
    // Spawn item box outline, does not depend on anything, simply the ui box.
    let item_outline = commands.spawn(NodeBundle {
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
    .id();

    // If player picks up item, render item image in ui outline.
    if let Ok(item) = item_q.get_single() {
        if item.in_inv {
            // Load font for text.
            let font_handle = asset_server.load("font/SourceCodePro.ttf");

            // Item image.
            let item_img = commands.spawn(ImageBundle {
                image: asset_server
                    .load(item.current_item.clone().unwrap().icon_path)
                    .into(),
                transform: Transform::from_scale(Vec3::new(2., 2., 0.)),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    item.current_item.clone().unwrap().name.to_string(),
                    TextStyle {
                        font: font_handle,
                        ..default()
                    }
                ));
            })
            .id();

            commands.entity(item_outline).add_child(item_img);
        }
    }
}

// Spawn idle item that player can pickup.
fn spawn_idle_item(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    item_res: Res<PlayerItems>,
) {
    // Make arbitrary item object.
    let item = Item::new(
        Vec3::new(-200., -50., 0.9),
        item_res.get("ice_cream".to_string()),
        false
    );

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(&item.current_item.as_ref().unwrap().icon_path),
            transform: Transform::from_translation(item.position),
            ..default()
        }
    ))
    .insert(item.clone())
    .insert(Sensor)
    .insert(ActiveEvents::COLLISION_EVENTS) // Necessary for Rapier to recieve collision events.
    .insert(Collider::cuboid(16., 16.)); // Item sprites are 32x32.
}

// Check if player has "picked up" (collided with) and item.
fn item_pickup(
    mut commands: Commands,
    rapier_context: Res<RapierContext>, 
    player_entity_q: Query<Entity, With<Player>>,
    item_entity_q: Query<Entity, With<Item>>,
    mut item_q: Query<&mut Item>
) {
    let player_entity= player_entity_q.single();

    if let Ok(mut item) = item_q.get_single_mut() {
        if let Ok(item_entity) = item_entity_q.get_single() {
            if rapier_context.intersection_pair(player_entity, item_entity) == Some(true) {
                item.in_inv = true;
            }
        }
    }
}