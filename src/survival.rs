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

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

#[derive(Message)]
pub enum Consume {
    Eat,
    Drink,
}

const STARVATION_DAMAGE: f32 = 2.0;
const EAT_AMOUNT: f32 = 40.0;
const DRINK_AMOUNT: f32 = 40.0;

pub struct SurvivalPlugin;

impl Plugin for SurvivalPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Consume>()
            .add_systems(Startup, spawn_hud)
            .add_systems(
                Update,
                (
                    decay_needs,
                    read_consume_input,
                    apply_consume,
                    apply_starvation,
                    update_hud,
                )
                    .chain(),
            );
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

fn read_consume_input(keys: Res<ButtonInput<KeyCode>>, mut writer: MessageWriter<Consume>) {
    if keys.just_pressed(KeyCode::KeyE) {
        writer.write(Consume::Eat);
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        writer.write(Consume::Drink);
    }
}

fn apply_consume(mut reader: MessageReader<Consume>, needs: Single<&mut Needs, With<Player>>) {
    let mut needs = needs.into_inner();
    for action in reader.read() {
        match action {
            Consume::Eat => {
                needs.hunger = (needs.hunger + EAT_AMOUNT).min(100.0);
            }
            Consume::Drink => {
                needs.thirst = (needs.thirst + DRINK_AMOUNT).min(100.0);
            }
        }
    }
}

fn apply_starvation(time: Res<Time>, player: Single<(&Needs, &mut Health), With<Player>>) {
    let (needs, mut health) = player.into_inner();
    if needs.hunger <= 0.0 || needs.thirst <= 0.0 {
        health.current = (health.current - STARVATION_DAMAGE * time.delta_secs()).max(0.0);
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

fn update_hud(
    player: Single<(&Needs, &Health), With<Player>>,
    hud: Single<&mut Text, With<NeedsHud>>,
) {
    let (needs, health) = player.into_inner();
    let mut text = hud.into_inner();
    text.0 = format!(
        "Health: {:.0}/{:.0}\nHunger: {:.0}\nThirst: {:.0}\nEnergy: {:.0}\n[E] eat  [Q] drink",
        health.current, health.max, needs.hunger, needs.thirst, needs.energy
    );
}
