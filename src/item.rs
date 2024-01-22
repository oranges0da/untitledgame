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

#[derive(Component, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub icon_path: String,
    pub index: i32,
    pub in_inv: bool,
}

impl Item {
    pub fn new(
        name: String,
        icon_path: String,
        index: i32,
        in_inv: bool,
    ) -> Self {
        Item {
            name,
            icon_path,
            index,
            in_inv,
        }
    }

    pub fn empty() -> Self {
        Item {
            name: "".to_string(),
            icon_path: "".to_string(),
            index: 0,
            in_inv: false,
        }
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
                name: "Ice Cream".to_string(),
                icon_path: "item/food/ice_cream.png".to_string(),
                index: 0,
                in_inv: false,
            },
        );

        items.add(
            "soda".to_string(),
            Item {
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
    let Some(soda_item) = item_res.get("soda".to_string()) else { return; };
    let Some(ice_cream_item) = item_res.get("ice_cream".to_string()) else { return; };

    let ice_cream_entity = commands.spawn(
        SpriteBundle {
            texture: asset_server.load(ice_cream_item.icon_path.to_string()),
            transform: Transform {
                translation: Vec3::new(200., -50., 1.1),
                scale: Vec3::new(1., 1., 0.),
                ..default()
            },
            ..default()
        }
    )
    .insert(ice_cream_item.clone())
    .insert(Sensor)
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(Collider::cuboid(16., 16.))
    .id();

    let soda_entity = commands.spawn(
        SpriteBundle {
            texture: asset_server.load(soda_item.icon_path.to_string()),
            transform: Transform {
                translation: Vec3::new(-200., -50., 1.1),
                scale: Vec3::new(1., 1., 0.),
                ..default()
            },
            ..default()
        }
    )
    .insert(soda_item.clone())
    .insert(Sensor)
    .insert(ActiveEvents::COLLISION_EVENTS) // Necessary for Rapier to recieve collision events.
    .insert(Collider::cuboid(16., 16.)) // Item sprites are 32x32.
    .id();

    info!("Ice cream id: {:?}", ice_cream_entity);
    info!("Soda id: {:?}", soda_entity);
}

// Check if player has "picked up" (collided with) an item.
fn item_pickup(
    mut commands: Commands,
    player_entity_q: Query<Entity, With<Player>>,
    item_entity_q: Query<Entity, With<Item>>,
    mut item_q: Query<&mut Item>,
    rapier_context: Res<RapierContext>,
) {
    let player_entity= player_entity_q.single();

    for item_entity in item_entity_q.iter() {
        if rapier_context.intersection_pair(player_entity, item_entity) == Some(true) {
            // Get associated item component from intersected entity.
            if let Ok(mut item) = item_q.get_component_mut::<Item>(item_entity) {
                item.in_inv = true;
            }
        }
    }
}