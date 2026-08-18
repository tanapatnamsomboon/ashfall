use bevy::prelude::*;

use crate::iso::{DepthSorted, TILE_HEIGHT, grid_to_world};
use crate::player::Player;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConsumeKind {
    Food,
    Water,
}

#[derive(Deserialize, Clone, Default)]
pub struct Restore {
    #[serde(default)]
    pub hunger: f32,
    #[serde(default)]
    pub thirst: f32,
    #[serde(default)]
    pub energy: f32,
    #[serde(default)]
    pub health: f32,
}

#[derive(Deserialize, Clone)]
pub struct ItemDef {
    pub name: String,
    pub kind: ConsumeKind,
    #[serde(default)]
    pub restore: Restore,
    pub color: [f32; 3],
}

#[derive(Resource)]
pub struct ItemDb(pub HashMap<String, ItemDef>);

#[derive(Component)]
pub struct Item(pub String);

#[derive(Component, Default)]
pub struct Inventory {
    pub items: Vec<String>,
}

const ITEM_SPAWNS: &[(i32, i32, &str)] = &[
    (7, 5, "canned_food"),
    (6, 7, "water_bottle"),
    (8, 8, "canned_food"),
    (2, 8, "water_bottle"),
];

const PICKUP_RADIUS: f32 = 20.0;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_items, spawn_items).chain())
            .add_systems(Update, pickup_items);
    }
}

fn load_items(mut commands: Commands) {
    let text = std::fs::read_to_string("assets/items.json")
        .expect("can not read assets/items.json (did you create it?)");
    let defs: HashMap<String, ItemDef> =
        serde_json::from_str(&text).expect("invalid format in items.json");
    commands.insert_resource(ItemDb(defs));
}

fn spawn_items(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    db: Res<ItemDb>,
) {
    let mush = meshes.add(Rectangle::new(TILE_HEIGHT * 0.4, TILE_HEIGHT * 0.4));
    for &(x, y, id) in ITEM_SPAWNS {
        let def = db.0.get(id).expect("missing ID in items.json");
        let color = Color::srgb(def.color[0], def.color[1], def.color[2]);
        let pos = grid_to_world(x as f32 + 0.5, y as f32 + 0.5, 0.0);
        commands.spawn((
            Item(id.to_string()),
            Mesh2d(mush.clone()),
            MeshMaterial2d(materials.add(color)),
            Transform::from_translation(pos.extend(0.0)),
            DepthSorted { anchor_offset: 0.0 },
        ));
    }
}

fn pickup_items(
    mut commands: Commands,
    player: Single<(&Transform, &mut Inventory), With<Player>>,
    items: Query<(Entity, &Transform, &Item), Without<Player>>,
) {
    let (player_tf, mut inventory) = player.into_inner();
    let player_pos = player_tf.translation.truncate();

    for (entity, item_tf, item) in &items {
        if player_pos.distance(item_tf.translation.truncate()) < PICKUP_RADIUS {
            inventory.items.push(item.0.clone());
            commands.entity(entity).despawn();
        }
    }
}
