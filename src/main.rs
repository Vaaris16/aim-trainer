use avian3d::PhysicsPlugins;
use bevy::{camera_controller::free_camera::FreeCameraPlugin, prelude::*};

use crate::{end_game::end_game::EndGamePlugin, game::game::GamePlugin};

mod end_game;
mod game;
mod splashscreen;

use splashscreen::splashscreen::SplashScreenPlugin;

pub const UI_BACKGROUND_COLOR: Color = Color::hsl(212., 0.25, 0.13);
pub const ACCENT_COLOR: Color = Color::hsl(199., 0.95, 0.75);
pub const TEXT_COLOR: Color = Color::WHITE;

#[derive(Default, States, Hash, Debug, PartialEq, Eq, Clone)]
enum GameState {
    #[default]
    SplashScreen,
    Game,
    EndGame,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FreeCameraPlugin)
        .add_plugins(PhysicsPlugins::default())
        .init_state::<GameState>()
        .add_plugins((SplashScreenPlugin, GamePlugin, EndGamePlugin))
        .run();
}
