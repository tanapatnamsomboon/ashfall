use bevy::prelude::*;

use crate::state::GameState;

const GAME_MINUTES_PER_SECOND: f32 = 30.0;
const MINUTES_PER_DAY: f32 = 24.0 * 60.0;
const MAX_NIGHT_ALPHA: f32 = 0.6;

#[derive(Resource, Default)]
pub struct GameClock {
    minutes: f32,
}

impl GameClock {
    fn day_minutes(&self) -> f32 {
        self.minutes % MINUTES_PER_DAY
    }

    fn day(&self) -> u32 {
        (self.minutes / MINUTES_PER_DAY) as u32 + 1
    }

    fn hour_min(&self) -> (u32, u32) {
        let m = self.day_minutes();
        ((m / 60.0) as u32, (m % 60.0) as u32)
    }

    fn darkness(&self) -> f32 {
        let t = self.day_minutes() / MINUTES_PER_DAY;
        let daylight = (1.0 - (t * std::f32::consts::TAU).cos()) * 0.5;
        (1.0 - daylight).clamp(0.0, 1.0)
    }
}

#[derive(Component)]
struct NightOverlay;

#[derive(Component)]
struct TimeText;

pub struct DayNightPlugin;

impl Plugin for DayNightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameClock>()
            .add_systems(Startup, (spawn_night_overlay, spawn_time_text))
            .add_systems(Update, advance_time.run_if(in_state(GameState::Playing)))
            .add_systems(Update, (update_night, update_time_text));
    }
}

fn advance_time(time: Res<Time>, mut clock: ResMut<GameClock>) {
    clock.minutes += GAME_MINUTES_PER_SECOND * time.delta_secs();
}

fn spawn_night_overlay(mut commands: Commands) {
    commands.spawn((
        NightOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.03, 0.10, 0.0)),
        GlobalZIndex(-1),
    ));
}

fn update_night(clock: Res<GameClock>, overlay: Single<&mut BackgroundColor, With<NightOverlay>>) {
    let mut bg = overlay.into_inner();
    bg.0 = Color::srgba(0.02, 0.03, 0.10, clock.darkness() * MAX_NIGHT_ALPHA);
}

fn spawn_time_text(mut commands: Commands) {
    commands.spawn((
        TimeText,
        Text::new(""),
        TextFont::from_font_size(20.0),
        TextColor(Color::srgb(0.92, 0.92, 0.92)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            right: Val::Px(12.0),
            ..default()
        },
    ));
}

fn update_time_text(clock: Res<GameClock>, text: Single<&mut Text, With<TimeText>>) {
    let (h, m) = clock.hour_min();
    let mut text = text.into_inner();
    text.0 = format!("Day {} {:02}:{:02}", clock.day(), h, m);
}
