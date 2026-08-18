use bevy::prelude::*;

mod camera;
mod game;
mod iso;
mod item;
mod player;
mod survival;
mod world;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ashfall".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.12)))
        .add_plugins(GamePlugin)
        .run();
}
