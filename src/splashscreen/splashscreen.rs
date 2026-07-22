use bevy::{prelude::*, text::LetterSpacing};

use crate::{
    ACCENT_COLOR, GameState, TEXT_COLOR, UI_BACKGROUND_COLOR,
    fonts::fonts::InterFonts,
    game::{
        targets::target::cleanup_targets, ui::timer::timer::reset_score,
        utilities::change_free_camera::disable_free_cam,
    },
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct SplashSet;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, SplashSet.run_if(in_state(GameState::SplashScreen)));
        app.add_systems(OnEnter(GameState::SplashScreen), spawn_splashscreen)
            .add_systems(Update, disable_free_cam.in_set(SplashSet))
            .add_systems(OnEnter(GameState::SplashScreen), reset_score)
            .add_systems(OnEnter(GameState::SplashScreen), cleanup_targets)
            .add_systems(OnEnter(GameState::Game), cleanup_splash)
            .add_systems(Update, button_interactions.in_set(SplashSet));
    }
}

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
struct SplashScreenRoot;

fn spawn_splashscreen(mut commands: Commands, fonts: Res<InterFonts>) {
    commands
        .spawn((
            Node {
                width: percent(100.),
                height: percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(20),
                ..Default::default()
            },
            SplashScreenRoot,
        ))
        .with_children(|parent| {
            // Text
            parent
                .spawn((Node {
                    width: px(300),
                    height: auto(),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: px(-30),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },))
                .with_children(|text| {
                    text.spawn((
                        Text::new("AIM"),
                        TextFont {
                            font_size: FontSize::Px(132.),
                            font: FontSource::Handle(fonts.inter_medium.clone()),
                            ..Default::default()
                        },
                        TextColor(TEXT_COLOR),
                        LetterSpacing::Px(20.),
                    ));
                    text.spawn((
                        Text::new("Trainer"),
                        TextFont {
                            font_size: FontSize::Px(64.),
                            font: FontSource::Handle(fonts.inter_medium.clone()),
                            ..Default::default()
                        },
                        TextColor(TEXT_COLOR),
                        LetterSpacing::Px(15.),
                    ));
                });
            // Button
            parent
                .spawn((
                    Node {
                        width: px(300),
                        height: px(75),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(10)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor::all(ACCENT_COLOR),
                    BackgroundColor(UI_BACKGROUND_COLOR),
                    Button,
                    StartButton,
                    BoxShadow(vec![ShadowStyle {
                        color: ACCENT_COLOR,
                        spread_radius: px(1),
                        blur_radius: px(1),
                        x_offset: px(0),
                        y_offset: px(0),
                        ..Default::default()
                    }]),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("START"),
                        TextColor(TEXT_COLOR),
                        TextFont {
                            font_size: FontSize::Px(32.),
                            font: FontSource::Handle(fonts.inter_medium.clone()),
                            ..Default::default()
                        },
                    ));
                });
        });
}

fn cleanup_splash(mut commands: Commands, splash_screen: Query<Entity, With<SplashScreenRoot>>) {
    for splash_screen in splash_screen {
        commands.entity(splash_screen).despawn();
    }
}

fn button_interactions(
    mut state: ResMut<NextState<GameState>>,
    interaction_button: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
) {
    for interaction in interaction_button {
        match *interaction {
            Interaction::Pressed => {
                state.set(GameState::Game);
            }
            _ => (),
        }
    }
}
