use crate::{
    ACCENT_COLOR, GameState, TEXT_COLOR, UI_BACKGROUND_COLOR, fonts::fonts_plugin::InterFonts,
    utilities::spawn_text::SpawnText,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource, Debug)]
pub struct Score(pub i32);

pub struct ScorePlugins;

#[derive(SystemSet, Default, Debug, Clone, PartialEq, Eq, Hash)]
struct ScoreSet;

impl Plugin for ScorePlugins {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, ScoreSet.run_if(in_state(GameState::Game)))
            .insert_resource(Score(0))
            .add_systems(OnEnter(GameState::Game), spawn_score)
            .add_systems(OnExit(GameState::Game), despawn_score)
            .add_systems(Update, change_score.in_set(ScoreSet));
    }
}

#[derive(Component)]
struct ScoreComponent;

pub fn spawn_score(mut commands: Commands, fonts: Res<InterFonts>) {
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
                right: px(15),

                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,

                border: UiRect::all(px(3)),
                border_radius: BorderRadius::all(px(12)),
                ..Default::default()
            },
            ScoreComponent,
            BorderColor::all(ACCENT_COLOR),
            BackgroundColor(UI_BACKGROUND_COLOR),
            BoxShadow(vec![ShadowStyle {
                color: ACCENT_COLOR,
                spread_radius: px(1),
                blur_radius: px(5),
                x_offset: px(0),
                y_offset: px(0),
            }]),
        ))
        .with_children(|parent| {
            parent.spawn(
                SpawnText::new(
                    String::from("Score"),
                    fonts.inter_medium.clone(),
                    None,
                    TEXT_COLOR,
                    None,
                )
                .spawn_text(),
            );

            parent.spawn((
                SpawnText::new(
                    String::from("0"),
                    fonts.inter_medium.clone(),
                    Some(48.),
                    TEXT_COLOR,
                    Some(0.),
                )
                .spawn_text(),
                ScoreText,
            ));
        });
}

fn change_score(score: ResMut<Score>, score_text: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in score_text {
            text.0 = score.0.to_string();
        }
    }
}

fn despawn_score(mut commands: Commands, score: Query<Entity, With<ScoreComponent>>) {
    for entity in score {
        commands.entity(entity).despawn();
    }
}
