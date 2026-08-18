use crate::iso::IsoPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((IsoPlugin, WorldPlugin, PlayerPlugin));
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
