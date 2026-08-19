use bevy::prelude::*;

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Playing,
    Dead,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}
