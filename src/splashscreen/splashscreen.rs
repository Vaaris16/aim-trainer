use bevy::{
    asset::{AssetsMutIterator, saver::AssetSaver},
    camera_controller::free_camera::FreeCamera,
    prelude::*,
    text::LineHeight,
};

use crate::{ACCENT_COLOR, GameState, game::player::player::Player};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct SplashSet;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, SplashSet.run_if(in_state(GameState::SplashScreen)));
        app.add_systems(OnEnter(GameState::SplashScreen), spawn_splashscreen)
            .add_systems(OnEnter(GameState::SplashScreen), spawn_camera)
            .add_systems(OnEnter(GameState::Game), cleanup_splash)
            .add_systems(Update, button_interactions.in_set(SplashSet));
    }
}

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct SplashScreenCamera;

#[derive(Component)]
struct SplashScreenRoot;

fn spawn_splashscreen(mut commands: Commands, assets_server: ResMut<AssetServer>) {
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
            BackgroundColor(Color::BLACK),
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
                            font_size: 132.,
                            ..Default::default()
                        },
                    ));
                    text.spawn((
                        Text::new("Trainer"),
                        TextFont {
                            font_size: 64.,
                            ..Default::default()
                        },
                        TextColor(ACCENT_COLOR),
                    ));
                });
            // Button
            parent
                .spawn((
                    Node {
                        width: px(300),
                        height: px(75),
                        border: UiRect::all(px(1)),
                        border_radius: BorderRadius::all(px(10)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor::all(ACCENT_COLOR),
                    BackgroundColor(Color::BLACK),
                    Button,
                    StartButton,
                    BoxShadow(vec![ShadowStyle {
                        color: ACCENT_COLOR,
                        spread_radius: px(1),
                        blur_radius: px(5),
                        x_offset: px(0),
                        y_offset: px(0),
                        ..Default::default()
                    }]),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("START "),
                        TextColor(Color::WHITE),
                        TextFont {
                            font_size: 32.,
                            ..Default::default()
                        },
                    ));
                    button.spawn((ImageNode::new(assets_server.load("arrow.png")),));
                });
        });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), SplashScreenCamera));
}

fn cleanup_splash(
    mut commands: Commands,
    splashscreen_cam: Query<Entity, With<SplashScreenCamera>>,
    splash_screen: Query<Entity, With<SplashScreenRoot>>,
) {
    for cam in splashscreen_cam {
        commands.entity(cam).despawn();
    }

    for splash_screen in splash_screen {
        commands.entity(splash_screen).despawn();
    }
}

fn button_interactions(
    mut state: ResMut<NextState<GameState>>,
    interaction_button: Query<(&Interaction, &mut Node), Changed<Interaction>>,
) {
    for (interaction, mut button_style) in interaction_button {
        match *interaction {
            Interaction::Pressed => {
                state.set(GameState::Game);
            }
            _ => (),
        }
    }
}
