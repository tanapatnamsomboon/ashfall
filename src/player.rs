use bevy::prelude::*;
use crate::iso::{grid_to_world, TILE_HEIGHT};

const PLAYER_SPEED : f32 = 200.0;
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

    if keys.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { dir.x += 1.0; }

    let mut intent = intent.into_inner();
    intent.0 = dir.normalize_or_zero();
}

fn apply_movement(
    time: Res<Time>,
    player: Single<(&MoveIntent, &mut Transform), With<Player>>,
) {
    let (intent, mut transform) = player.into_inner();
    let step = intent.0 * PLAYER_SPEED * time.delta_secs();
    transform.translation += step.extend(0.0);
}