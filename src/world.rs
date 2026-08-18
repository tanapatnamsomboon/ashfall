use crate::iso::{DepthSorted, TILE_HEIGHT, TILE_WIDTH, grid_to_world};
use bevy::prelude::*;
use std::collections::HashSet;

const GRID_SIZE: i32 = 10;
const WALL_WIDTH: f32 = 24.0;
const WALL_HEIGHT: f32 = 56.0;
const WALL_TILES: &[(i32, i32)] = &[(3, 3), (4, 3), (5, 3), (3, 4), (3, 5)];

#[allow(dead_code)]
#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Resource)]
pub struct WorldGrid {
    pub size: IVec2,
    pub solids: HashSet<IVec2>,
}

impl WorldGrid {
    pub fn is_blocked(&self, tile: IVec2) -> bool {
        tile.x < 0
            || tile.y < 0
            || tile.x >= self.size.x
            || tile.y >= self.size.y
            || self.solids.contains(&tile)
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldGrid {
            size: IVec2::new(GRID_SIZE, GRID_SIZE),
            solids: HashSet::new(),
        })
        .add_systems(Startup, (spawn_ground, spawn_walls));
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
            let material = if (x + y) % 2 == 0 {
                light.clone()
            } else {
                dark.clone()
            };

            commands.spawn((
                Tile { x, y, z: 0 },
                Mesh2d(diamond.clone()),
                MeshMaterial2d(material),
                Transform::from_translation(pos.extend(0.0)),
            ));
        }
    }
}

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut grid: ResMut<WorldGrid>,
) {
    let wall_mesh = meshes.add(Rectangle::new(WALL_WIDTH, WALL_HEIGHT));
    let wall_mat = materials.add(Color::srgb(0.45, 0.40, 0.35));
    let half_h = WALL_HEIGHT / 2.0;

    for &(x, y) in WALL_TILES {
        grid.solids.insert(IVec2::new(x, y));
        let base = grid_to_world(x as f32 + 0.5, y as f32 + 0.5, 0.0);
        commands.spawn((
            Mesh2d(wall_mesh.clone()),
            MeshMaterial2d(wall_mat.clone()),
            Transform::from_translation(Vec3::new(base.x, base.y + half_h, 0.0)),
            DepthSorted {
                anchor_offset: -half_h,
            },
        ));
    }
}
