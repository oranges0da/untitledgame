use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;
use crate::player::Player;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Items>()
            .add_systems(Startup, spawn_idle_item)
            .add_systems(Update, show_item_ui)
            .add_systems(Update, item_pickup);
    }
}

#[derive(Component, Clone)]
pub struct Item {
    pub position: Vec3,
    pub name: String,
    pub icon_path: String,
    pub index: i32,
    pub in_inv: bool,
}

impl Item {
    pub fn new(
        position: Vec3,
        name: String,
        icon_path: String,
        index: i32,
        in_inv: bool,
    ) -> Self {
        Item {
            position,
            name,
            icon_path,
            index,
            in_inv,
        }
    }

    pub fn get_with_name(&self) -> Self {
        self.clone()
    }
}

#[derive(Resource)]
pub struct Items {
    items: HashMap<String, Item>,
}

impl Items {
    pub fn add(&mut self, id: String, item: Item) {
        self.items.insert(id, item);
    }

    pub fn get(&self, id: String) -> Option<Item> {
        return self.items.get(&id).cloned();
    }
}

// init items for global resource
impl FromWorld for Items {
    fn from_world(_world: &mut World) -> Self {
        let mut items = Items {
            items: HashMap::new(),
        };

        items.add(
            "ice_cream".to_string(),
            Item {
                position: Vec3::ZERO,
                name: "Ice Cream".to_string(),
                icon_path: "item/food/ice_cream.png".to_string(),
                index: 0,
                in_inv: false,
            },
        );

        items.add(
            "soda".to_string(),
            Item {
                position: Vec3::ZERO,
                name: "Soda".to_string(),
                icon_path: "item/food/soda.png".to_string(),
                index: 0,
                in_inv: false,
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
            // Font for text.
            let font_handle = asset_server.load("font/SourceCodePro.ttf");

            // Item image.
            let item_img = commands.spawn(ImageBundle {
                image: asset_server
                    .load(item.icon_path.to_string())
                    .into(),
                transform: Transform::from_scale(Vec3::new(2., 2., 0.)),
                ..default()
            })
            .with_children(|parent| { // Spawn item name.
                parent.spawn(TextBundle::from_section(
                    item.name.to_string(),
                    TextStyle {
                        font: font_handle,
                        ..default()
                    })
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(5.0),
                        right: Val::Px(5.0),
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
    item_res: Res<Items>,
) {
    let Some(item) = item_res.get("soda".to_string()) else { return; };

    commands.spawn(
        SpriteBundle {
            texture: asset_server.load(item.icon_path.to_string()),
            transform: Transform {
                translation: Vec3::new(-200., -50., 0.5),
                scale: Vec3::new(1., 1., 0.),
                ..default()
            },
            ..default()
        }
    )
    .insert(item.clone())
    .insert(Sensor)
    .insert(ActiveEvents::COLLISION_EVENTS) // Necessary for Rapier to recieve collision events.
    .insert(Collider::cuboid(16., 16.)); // Item sprites are 32x32.
}
// Check if player has "picked up" (collided with) and item.
fn item_pickup(
    player_entity_q: Query<Entity, With<Player>>,
    item_entity_q: Query<Entity, With<Item>>,
    mut item_q: Query<&mut Item>,
    rapier_context: Res<RapierContext>,
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