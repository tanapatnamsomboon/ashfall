use bevy::prelude::*;

use crate::iso::{DepthSorted, TILE_HEIGHT, grid_to_world};
use crate::player::Player;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Food,
    Water,
}

impl ItemKind {
    fn color(&self) -> Color {
        match self {
            ItemKind::Food => Color::srgb(0.85, 0.75, 0.35),
            ItemKind::Water => Color::srgb(0.30, 0.55, 0.90),
        }
    }
}

#[derive(Component)]
pub struct Item(pub ItemKind);

#[derive(Component, Default)]
pub struct Inventory {
    pub items: Vec<ItemKind>,
}

const ITEM_SPAWNS: &[(i32, i32, ItemKind)] = &[
    (7, 5, ItemKind::Food),
    (6, 7, ItemKind::Water),
    (8, 8, ItemKind::Food),
    (2, 8, ItemKind::Water),
];

const PICKUP_RADIUS: f32 = 20.0;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_items)
            .add_systems(Update, pickup_items);
    }
}

fn spawn_items(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mush = meshes.add(Rectangle::new(TILE_HEIGHT * 0.4, TILE_HEIGHT * 0.4));
    for &(x, y, kind) in ITEM_SPAWNS {
        let pos = grid_to_world(x as f32 + 0.5, y as f32 + 0.5, 0.0);
        commands.spawn((
            Item(kind),
            Mesh2d(mush.clone()),
            MeshMaterial2d(materials.add(kind.color())),
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
            inventory.items.push(item.0);
            commands.entity(entity).despawn();
        }
    }
}
