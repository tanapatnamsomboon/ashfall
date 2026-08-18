use bevy::prelude::*;
use crate::iso::{grid_to_world, TILE_HEIGHT};

const PLAYER_START: IVec2 = IVec2::new(5, 5);
const PLAYER_Z: f32 = 1.0;

#[derive(Component)]
pub struct Player {
    pub grid: IVec2,
}

#[derive(Message)]
pub struct MoveCommand {
    pub delta: IVec2,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MoveCommand>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (read_movement_input, apply_movement).chain());
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pos = grid_to_world(PLAYER_START.x as f32 + 0.5, PLAYER_START.y as f32 + 0.5, 0.0);
    commands.spawn((
        Player { grid: PLAYER_START },
        Mesh2d(meshes.add(Circle::new(TILE_HEIGHT * 0.35))),
        MeshMaterial2d(materials.add(Color::srgb(0.90, 0.50, 0.20))),
        Transform::from_translation(pos.extend(PLAYER_Z)),
    ));
}

fn read_movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: MessageWriter<MoveCommand>,
) {
    let mut delta = IVec2::ZERO;

    if keys.just_pressed(KeyCode::KeyW) { delta += IVec2::new(0, -1); }
    if keys.just_pressed(KeyCode::KeyS) { delta += IVec2::new(0, 1); }
    if keys.just_pressed(KeyCode::KeyA) { delta += IVec2::new(-1, 0); }
    if keys.just_pressed(KeyCode::KeyD) { delta += IVec2::new(1, 0); }

    if delta != IVec2::ZERO {
        writer.write(MoveCommand { delta });
    }
}

fn apply_movement(
    mut reader: MessageReader<MoveCommand>,
    player: Single<(&mut Player, &mut Transform)>,
) {
    let (mut player_delta, mut transform) = player.into_inner();
    for cmd in reader.read() {
        player_delta.grid += cmd.delta;
        let pos = grid_to_world(
            player_delta.grid.x as f32 + 0.5,
            player_delta.grid.y as f32 + 0.5,
            0.0
        );
        transform.translation = pos.extend(PLAYER_Z);
    }
}