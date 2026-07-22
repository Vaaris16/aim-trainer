use crate::{
    ACCENT_COLOR, GameState, UI_BACKGROUND_COLOR,
    game::{ui::score::score::Score, utilities::change_free_camera::disable_free_cam},
};
use bevy::prelude::*;

#[derive(SystemSet, Hash, PartialEq, Eq, Debug, Clone)]
struct EndGameSet;

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, EndGameSet.run_if(in_state(GameState::EndGame)))
            .add_systems(OnEnter(GameState::EndGame), spawn_ui)
            .add_systems(OnExit(GameState::EndGame), clean_up_endgame)
            .add_systems(Update, disable_free_cam.in_set(EndGameSet))
            .add_systems(Update, button_interactions.in_set(EndGameSet));
    }
}

#[derive(Component)]
struct EndGameRoot;

#[derive(Component)]
struct RestartButton;

fn spawn_ui(assets_server: Res<AssetServer>, mut commands: Commands, score: Res<Score>) {
    let inter_medium: Handle<Font> = assets_server.load("fonts/inter/static/Inter_24pt-Medium.ttf");

    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            EndGameRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: px(400),
                        height: px(500),

                        border_radius: BorderRadius::all(px(13)),
                        border: UiRect::all(px(3)),

                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,

                        padding: UiRect::all(px(20)),
                        ..Default::default()
                    },
                    BackgroundColor(UI_BACKGROUND_COLOR),
                    BorderColor::all(ACCENT_COLOR),
                    BoxShadow(vec![ShadowStyle {
                        color: ACCENT_COLOR,
                        spread_radius: px(1),
                        blur_radius: px(5),
                        x_offset: px(0),
                        y_offset: px(0),
                        ..Default::default()
                    }]),
                ))
                .with_children(|main_box| {
                    // Spawn "score" text
                    main_box.spawn((
                        Text::new("Final Score"),
                        TextColor(ACCENT_COLOR),
                        TextFont {
                            font_size: FontSize::Px(25.),
                            font: FontSource::Handle(inter_medium.clone()),
                            ..Default::default()
                        },
                    ));

                    // Spawn score
                    main_box.spawn((
                        Text::new(score.0.to_string()),
                        TextFont {
                            font_size: FontSize::Px(120.),
                            font: FontSource::Handle(inter_medium.clone()),
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::top(px(45.)),
                            ..default()
                        },
                    ));

                    // spawn button
                    main_box
                        .spawn((
                            Node {
                                width: px(180),
                                height: px(70),

                                margin: UiRect::top(px(90)),

                                border: UiRect::all(px(4)),
                                border_radius: BorderRadius::all(px(25)),

                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            Button,
                            RestartButton,
                            BackgroundColor(UI_BACKGROUND_COLOR),
                            BorderColor::all(ACCENT_COLOR),
                            BoxShadow(vec![ShadowStyle {
                                color: ACCENT_COLOR,
                                spread_radius: px(1),
                                blur_radius: px(2),
                                x_offset: px(0),
                                y_offset: px(0),
                                ..Default::default()
                            }]),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Restart"),
                                TextColor(ACCENT_COLOR),
                                TextFont {
                                    font_size: FontSize::Px(30.),
                                    font: FontSource::Handle(inter_medium.clone()),
                                    ..Default::default()
                                },
                            ));
                        });
                });
        });
}

fn button_interactions(
    button: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut state: ResMut<NextState<GameState>>,
) {
    for interaction in button {
        match *interaction {
            Interaction::Pressed => {
                state.set(GameState::SplashScreen);
            }
            _ => (),
        }
    }
}

fn clean_up_endgame(mut commands: Commands, entity: Query<Entity, With<EndGameRoot>>) {
    for end_game_entity in entity {
        commands.entity(end_game_entity).despawn();
    }
}
