use bevy::prelude::*;

use crate::item::{ConsumeKind, Inventory, ItemDb, Restore};
use crate::player::Player;
use crate::state::GameState;

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
            .add_systems(Update, update_hud)
            .add_systems(
                Update,
                (
                    decay_needs,
                    read_consume_input,
                    apply_consume,
                    apply_starvation,
                    check_death,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::Dead), show_death_screen)
            .add_systems(OnExit(GameState::Dead), despawn_death_screen)
            .add_systems(Update, restart.run_if(in_state(GameState::Dead)));
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

fn apply_restore(r: &Restore, needs: &mut Needs, health: &mut Health) {
    needs.hunger = (needs.hunger + r.hunger).min(100.0);
    needs.thirst = (needs.thirst + r.thirst).min(100.0);
    needs.energy = (needs.energy + r.energy).min(100.0);
    health.current = (health.current + r.health).min(health.max);
}

fn apply_consume(
    mut reader: MessageReader<Consume>,
    db: Res<ItemDb>,
    player: Single<(&mut Needs, &mut Health, &mut Inventory), With<Player>>,
) {
    let (mut needs, mut health, mut inventory) = player.into_inner();
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
                apply_restore(&def.restore, &mut needs, &mut health)
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

fn check_death(health: Single<&Health, With<Player>>, mut next: ResMut<NextState<GameState>>) {
    if health.current <= 0.0 {
        next.set(GameState::Dead);
    }
}

#[derive(Component)]
struct DeathScreen;

fn show_death_screen(mut commands: Commands) {
    commands
        .spawn((
            DeathScreen,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("YOU DIED\n [R] restart"),
                TextFont::from_font_size(44.0),
                TextColor(Color::srgb(0.85, 0.20, 0.20)),
            ));
        });
}

fn despawn_death_screen(mut commands: Commands, q: Query<Entity, With<DeathScreen>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn restart(
    keys: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Needs, &mut Health), With<Player>>,
    mut next: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        let (mut needs, mut health) = player.into_inner();
        *needs = Needs::default();
        *health = Health::default();
        next.set(GameState::Playing);
    }
}
