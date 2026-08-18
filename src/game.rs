use bevy::prelude::*;

use crate::camera::CameraPlugin;
use crate::iso::IsoPlugin;
use crate::item::ItemPlugin;
use crate::player::PlayerPlugin;
use crate::survival::SurvivalPlugin;
use crate::world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CameraPlugin,
            IsoPlugin,
            WorldPlugin,
            PlayerPlugin,
            SurvivalPlugin,
            ItemPlugin,
        ));
    }
}
