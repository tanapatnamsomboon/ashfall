use bevy::prelude::*;

use crate::iso::{DepthSorted, TILE_HEIGHT, grid_to_world, world_to_grid};
use crate::world::WorldGrid;

const PLAYER_SPEED: f32 = 100.0;
const PLAYER_Z: f32 = 1.0;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct MoveIntent(Vec2);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (read_movement_input, apply_movement).chain());
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pos = grid_to_world(5.5, 5.5, 0.0);
    commands.spawn((
        Player,
        MoveIntent::default(),
        DepthSorted { anchor_offset: 0.0 },
        Mesh2d(meshes.add(Circle::new(TILE_HEIGHT * 0.35))),
        MeshMaterial2d(materials.add(Color::srgb(0.90, 0.50, 0.20))),
        Transform::from_translation(pos.extend(PLAYER_Z)),
    ));
}

fn read_movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    intent: Single<&mut MoveIntent, With<Player>>,
) {
    let mut dir = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    let mut intent = intent.into_inner();
    intent.0 = dir.normalize_or_zero();
}

fn apply_movement(
    time: Res<Time>,
    grid: Res<WorldGrid>,
    player: Single<(&MoveIntent, &mut Transform), With<Player>>,
) {
    let (intent, mut transform) = player.into_inner();
    let step = intent.0 * PLAYER_SPEED * time.delta_secs();
    let pos = transform.translation.truncate();

    let mut new = pos;

    let try_x = Vec2::new(pos.x + step.x, pos.y);
    if !tile_blocked(try_x, &grid) {
        new.x = try_x.x;
    }

    let try_y = Vec2::new(new.x, pos.y + step.y);
    if !tile_blocked(try_y, &grid) {
        new.y = try_y.y;
    }

    transform.translation.x = new.x;
    transform.translation.y = new.y;
}

fn tile_blocked(world_pos: Vec2, grid: &WorldGrid) -> bool {
    let g = world_to_grid(world_pos, 0.0);
    let tile = IVec2::new(g.x.floor() as i32, g.y.floor() as i32);
    grid.is_blocked(tile)
}
