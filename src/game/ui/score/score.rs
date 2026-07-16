use crate::{GameState, UI_BACKGROUND_COLOR};
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
            .add_systems(Update, change_score.in_set(ScoreSet));
    }
}

pub fn spawn_score(mut commands: Commands) {
    commands
        .spawn((
            Node {
                padding: UiRect {
                    top: px(15),
                    bottom: px(15),
                    left: px(30),
                    right: px(30),
                },
                position_type: PositionType::Absolute,
                top: px(24),
                right: px(15),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                border_radius: BorderRadius::all(px(12)),
                ..Default::default()
            },
            BackgroundColor(UI_BACKGROUND_COLOR),
        ))
        .with_children(|parent| {
            parent.spawn((Text::new("Score"),));
            parent.spawn((
                Text::new("0"),
                TextFont {
                    font_size: FontSize::Px(48.),
                    ..Default::default()
                },
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
