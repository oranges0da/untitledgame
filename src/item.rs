use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;
use crate::player::Player;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Items>()
            .add_systems(Startup, spawn_idle_item)
            .add_systems(Startup, spawn_item_ui)
            .add_systems(Update, item_pickup)
            .add_systems(Update, drop_item)
            .add_systems(Update, update_item_ui);
    }
}

#[derive(Component, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub icon_path: String,
    pub in_inv: bool,
}

impl Item {
    pub fn new(
        name: String,
        icon_path: String,
        in_inv: bool,
    ) -> Self {
        Item {
            name,
            icon_path,
            in_inv,
        }
    }

    pub fn empty() -> Self {
        Item {
            name: "".to_string(),
            icon_path: "".to_string(),
            in_inv: false,
        }
    }
}

#[derive(Resource)]
struct Items {
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
                in_inv: false,
            },
        );

        items.add(
            "soda".to_string(),
            Item {
                name: "Soda".to_string(),
                icon_path: "item/food/soda.png".to_string(),
                in_inv: false,
            },
        );

        items
    }
}

// Spawn idle item that player can pickup.
fn spawn_idle_item(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    item_res: Res<Items>,
) {
    let Some(ice_cream_item) = item_res.get("ice_cream".to_string()) else { return; };
    let Some(soda_item) = item_res.get("soda".to_string()) else { return; };

    commands.spawn(
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
    .insert(ice_cream_item.clone());
    // .insert(Sensor)
    // .insert(ActiveEvents::COLLISION_EVENTS)
    // .insert(Collider::cuboid(16., 16.));

    // commands.spawn(
    //     SpriteBundle {
    //         texture: asset_server.load(soda_item.icon_path.to_string()),
    //         transform: Transform {
    //             translation: Vec3::new(-200., -50., 1.1),
    //             scale: Vec3::new(1., 1., 0.),
    //             ..default()
    //         },
    //         ..default()
    //     }
    // )
    // .insert(soda_item.clone())
    // .insert(Sensor)
    // .insert(ActiveEvents::COLLISION_EVENTS)
    // .insert(Collider::cuboid(16., 16.));
}

// Check if player has "picked up" (collided with) an item.
fn item_pickup(
    player_entity_q: Query<Entity, With<Player>>,
    item_entity_q: Query<Entity, With<Item>>,
    mut item_q: Query<&mut Item>,
    rapier_context: Res<RapierContext>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let player_entity= player_entity_q.single();

    for item_entity in item_entity_q.iter() {
        if rapier_context.intersection_pair(player_entity, item_entity) == Some(true)
            && keyboard_input.just_pressed(KeyCode::E)
        {
            // Get associated component from intersected item entity.
            if let Ok(mut item) = item_q.get_component_mut::<Item>(item_entity) {
                item.in_inv = true;
            }
        }
    }
}

fn drop_item(
    player_q: Query<&Transform, With<Player>>,
    mut item_q: Query<&mut Item>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let pos = player_q.single();

    for mut item in &mut item_q.iter_mut() {
        if item.in_inv && keyboard_input.just_pressed(KeyCode::Q) {
            item.in_inv = false;
        }
    }
}

#[derive(Component)]
struct ItemUI;

fn spawn_item_ui(
    mut commands: Commands,
) {
    // Spawn item box outline.
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(30.),
            height: Val::Px(30.),
            top: Val::Px(30.),
            left: Val::Px(30.),
            border: UiRect::all(Val::Px(2.)),
            padding: UiRect::all(Val::Px(30.)),
            ..default()
        },
        border_color: BorderColor(Color::WHITE),
        ..default()
    });
}

// Spawn and despawn current item in ui.
fn update_item_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    item_q: Query<&Item>,
    item_ui_q: Query<Entity, With<ItemUI>>,
) {
    for item in item_q.iter() {
        if item.in_inv {
            // Item image.
            commands.spawn(ImageBundle {
                image: asset_server.load(item.icon_path.to_string()).into(),
                transform: Transform {
                    scale: Vec3::new(2.5, 2.5, 0.),
                    ..default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(45.),
                    left: Val::Px(45.),
                    ..default()
                },
                ..default()
            })
            .insert(ItemUI);

            let font_handle = asset_server.load("font/SourceCodePro.ttf");
            // Item name.
            commands.spawn(TextBundle::from_section(
                item.name.to_string(),
                TextStyle {
                    font: font_handle,
                    ..default()
                })
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.),
                    left: Val::Px(10.),
                    ..default()
                },
            ))
            .insert(ItemUI);
        } else {
            for item_entity in item_ui_q.iter() {
                commands.entity(item_entity).despawn_recursive();
            }
        }
    }
}
