use bevy::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct Needs {
    pub hunger: f32,
    pub thirst: f32,
    pub energy: f32,
}

impl Default for Needs {
    fn default() -> Self {
        Self {
            hunger: 100.0,
            thirst: 100.0,
            energy: 100.0,
        }
    }
}

const HUNGER_RATE: f32 = 0.5;
const THIRST_RATE: f32 = 0.8;
const ENERGY_RATE: f32 = 0.3;

#[derive(Component)]
struct NeedsHud;

pub struct SurvivalPlugin;

impl Plugin for SurvivalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(Update, (decay_needs, update_hud).chain());
    }
}

fn decay_needs(time: Res<Time>, mut query: Query<&mut Needs>) {
    let dt = time.delta_secs();
    for mut n in &mut query {
        n.hunger = (n.hunger - HUNGER_RATE * dt).max(0.0);
        n.thirst = (n.thirst - THIRST_RATE * dt).max(0.0);
        n.energy = (n.energy - ENERGY_RATE * dt).max(0.0);
    }
}

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        NeedsHud,
        Text::new("Hunger: --\nThirst: --\nEnergy: --"),
        TextFont::from_font_size(20.0),
        TextColor(Color::srgb(0.92, 0.92, 0.92)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn update_hud(needs: Single<&Needs, With<Player>>, hud: Single<&mut Text, With<NeedsHud>>) {
    let mut text = hud.into_inner();
    text.0 = format!(
        "Hunger: {:.0}\nThirst: {:.0}\nEnergy: {:.0}",
        needs.hunger, needs.thirst, needs.energy
    );
}
