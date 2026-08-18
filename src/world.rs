use bevy::prelude::*;
use crate::iso::{grid_to_world, TILE_HEIGHT, TILE_WIDTH};

const GRID_SIZE: i32 = 10;

#[allow(dead_code)]
#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let diamond = meshes.add(Rhombus::new(TILE_WIDTH, TILE_HEIGHT));

    let light = materials.add(Color::srgb(0.28, 0.30, 0.26));
    let dark = materials.add(Color::srgb(0.22, 0.24, 0.20));

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let pos = grid_to_world(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            let material = if (x + y) % 2 == 0 { light.clone() } else { dark.clone() };

            commands.spawn((
                Tile { x, y, z: 0 },
                Mesh2d(diamond.clone()),
                MeshMaterial2d(material),
                Transform::from_translation(pos.extend(0.0)),
            ));
        }
    }
}