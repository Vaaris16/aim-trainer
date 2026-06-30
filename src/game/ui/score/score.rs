use crate::{ACCENT_COLOR, GameState};
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
                padding: UiRect::all(px(5)),
                position_type: PositionType::Absolute,
                top: px(24),
                right: px(15),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(12)),
                ..Default::default()
            },
            BorderColor::all(ACCENT_COLOR),
            BackgroundColor(Color::BLACK),
            BoxShadow(vec![ShadowStyle {
                color: ACCENT_COLOR,
                spread_radius: px(0.5),
                x_offset: px(0),
                y_offset: px(0),
                ..Default::default()
            }]),
        ))
        .with_children(|parent| {
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
