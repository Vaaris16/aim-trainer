use crate::ACCENT_COLOR;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource)]
pub struct Score(pub i32);

pub fn spawn_score(mut commands: Commands) {
    commands
        .spawn((
            Node {
                padding: UiRect::all(Val::Px(5.)),
                position_type: PositionType::Absolute,
                top: px(24.),
                right: px(15.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(px(1.)),
                border_radius: BorderRadius::all(px(12.)),
                ..Default::default()
            },
            BorderColor::all(ACCENT_COLOR),
            BackgroundColor(Color::BLACK),
            BoxShadow(vec![ShadowStyle {
                color: ACCENT_COLOR,
                spread_radius: px(0.5),
                blur_radius: px(2.),
                x_offset: px(0.),
                y_offset: px(0.),
                ..Default::default()
            }]),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("0"),
                TextFont {
                    font_size: 48.,
                    ..Default::default()
                },
                ScoreText,
            ));
        });
}

pub fn update_score(score_text: Query<&mut Text, With<ScoreText>>, mut score_query: ResMut<Score>) {
    score_query.0 += 50;

    for mut text in score_text {
        text.0 = score_query.0.to_string();
    }
}
