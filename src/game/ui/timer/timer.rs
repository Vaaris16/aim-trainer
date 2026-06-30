use std::{arch::aarch64::int16x4_t, time::Duration};

use bevy::prelude::*;

use crate::{ACCENT_COLOR, GameState};

#[derive(Resource, Debug)]
struct GameTimer(pub Timer);

pub struct TimerPlugin;

#[derive(SystemSet, Default, Debug, Clone, PartialEq, Eq, Hash)]
struct TimerSet;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, TimerSet.run_if(in_state(GameState::Game)))
            .add_systems(OnEnter(GameState::Game), spawn_timer)
            .insert_resource(GameTimer(Timer::new(
                Duration::from_secs(15),
                TimerMode::Once,
            )))
            .add_systems(Update, update_timer.in_set(TimerSet))
            .add_systems(Update, tick_timer.in_set(TimerSet));
    }
}

#[derive(Component)]
struct TimerUi;

fn tick_timer(time: Res<Time>, mut timer: ResMut<GameTimer>) {
    timer.0.tick(time.delta());
    println!("{:?}", time.delta());
}

fn spawn_timer(mut commands: Commands) {
    commands
        .spawn((
            Node {
                padding: UiRect::all(px(5)),
                top: px(24),
                left: px(15),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(12.)),
                ..Default::default()
            },
            BorderColor::all(ACCENT_COLOR),
            BackgroundColor(ACCENT_COLOR),
        ))
        .with_children(|timer_text| {
            timer_text.spawn((
                Text::new("15"),
                TextFont {
                    font_size: 48.,
                    ..Default::default()
                },
                TimerUi,
            ));
        });
}

fn update_timer(game_timer: Res<GameTimer>, timer_ui: Query<&mut Text, With<TimerUi>>) {
    if game_timer.is_changed() {
        let time_sec = game_timer.0.remaining_secs() as i32;

        for mut time in timer_ui {
            time.0 = time_sec.to_string();
            println!("{}", game_timer.0.remaining_secs());
        }
    }
}
