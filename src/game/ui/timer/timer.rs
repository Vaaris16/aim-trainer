use std::time::Duration;

use bevy::prelude::*;

use crate::{ACCENT_COLOR, GameState, UI_BACKGROUND_COLOR};

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
                Duration::from_secs(5),
                TimerMode::Once,
            )))
            .add_systems(Update, update_timer.in_set(TimerSet))
            .add_systems(Update, tick_timer.in_set(TimerSet))
            .add_systems(OnExit(GameState::Game), despawn_timer)
            .add_systems(Update, timer_end.in_set(TimerSet));
    }
}

#[derive(Component)]
struct TimerUi;

#[derive(Component)]
struct TimerComponent;

fn tick_timer(time: Res<Time>, mut timer: ResMut<GameTimer>) {
    timer.0.tick(time.delta());
}

fn spawn_timer(mut commands: Commands) {
    commands
        .spawn((
            Node {
                padding: UiRect {
                    top: px(15),
                    bottom: px(15),
                    left: px(30),
                    right: px(30),
                },
                top: px(24),
                left: px(15),

                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,

                border: UiRect::all(px(3)),
                border_radius: BorderRadius::all(px(12.)),
                ..Default::default()
            },
            TimerComponent,
            BorderColor::all(ACCENT_COLOR),
            BoxShadow(vec![ShadowStyle {
                color: ACCENT_COLOR,
                spread_radius: px(1),
                blur_radius: px(5),
                x_offset: px(0),
                y_offset: px(0),
                ..Default::default()
            }]),
            BackgroundColor(UI_BACKGROUND_COLOR),
        ))
        .with_children(|timer_text| {
            timer_text.spawn(Text::new("Time"));
            timer_text.spawn((
                Text::new("15"),
                TextFont {
                    font_size: FontSize::Px(48.),
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
        }
    }
}

fn timer_end(mut game_state: ResMut<NextState<GameState>>, game_timer: Res<GameTimer>) {
    if game_timer.0.is_finished() {
        game_state.set(GameState::EndGame);
    }
}

fn despawn_timer(mut commands: Commands, timer: Query<Entity, With<TimerComponent>>) {
    for entity in timer {
        commands.entity(entity).despawn();
    }
}
