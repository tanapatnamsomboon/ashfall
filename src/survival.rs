use bevy::prelude::*;

use crate::item::{ConsumeKind, Inventory, ItemDb};
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

fn apply_consume(
    mut reader: MessageReader<Consume>,
    db: Res<ItemDb>,
    player: Single<(&mut Needs, &mut Inventory), With<Player>>,
) {
    let (mut needs, mut inventory) = player.into_inner();
    for action in reader.read() {
        let want = match action {
            Consume::Eat => ConsumeKind::Food,
            Consume::Drink => ConsumeKind::Water,
        };
        let found = inventory
            .items
            .iter()
            .position(|id| db.0.get(id).map_or(false, |d| d.kind == want));
        if let Some(pos) = found {
            let id = inventory.items.remove(pos);
            if let Some(def) = db.0.get(&id) {
                match def.kind {
                    ConsumeKind::Food => needs.hunger = (needs.hunger + def.restore).min(100.0),
                    ConsumeKind::Water => needs.thirst = (needs.thirst + def.restore).min(100.0),
                }
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

fn count_kind(inventory: &Inventory, db: &ItemDb, kind: ConsumeKind) -> usize {
    inventory
        .items
        .iter()
        .filter(|id| db.0.get(*id).map_or(false, |d| d.kind == kind))
        .count()
}

fn update_hud(
    player: Single<(&Needs, &Health, &Inventory), With<Player>>,
    db: Res<ItemDb>,
    hud: Single<&mut Text, With<NeedsHud>>,
) {
    let (needs, health, inventory) = player.into_inner();
    let food = count_kind(inventory, &db, ConsumeKind::Food);
    let water = count_kind(inventory, &db, ConsumeKind::Water);
    let mut text = hud.into_inner();
    text.0 = format!(
        "Health: {:.0}/{:.0}\nHunger: {:.0}\nThirst: {:.0}\nEnergy: {:.0}\n\nFood: {food}   Water: {water}\n[E] eat  [Q] drink",
        health.current, health.max, needs.hunger, needs.thirst, needs.energy
    );
}
