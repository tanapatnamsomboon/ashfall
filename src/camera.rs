use bevy::prelude::*;

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn camera_follow(
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let mut camera_tf = camera.into_inner();
    camera_tf.translation.x = player.translation.x;
    camera_tf.translation.y = player.translation.y;
}
