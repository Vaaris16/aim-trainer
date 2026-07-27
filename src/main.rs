#![allow(clippy::type_complexity)]
use avian3d::PhysicsPlugins;
use bevy::{
    camera_controller::free_camera::FreeCameraPlugin, prelude::*,
    render::render_resource::TextureSampleType,
};
use bevy_tape::TapePlugin;

mod end_game;
mod fonts;
mod game;
mod splashscreen;
mod utilities;

use crate::{
    end_game::end_game_plugin::EndGamePlugin, fonts::fonts_plugin::FontsPlugin,
    game::game_plugin::GamePlugin, splashscreen::splashscreen_plugin::SplashScreenPlugin,
};

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
        .add_plugins(TapePlugin)
        .add_plugins(FontsPlugin)
        .add_plugins(PhysicsPlugins::default())
        .init_state::<GameState>()
        .add_plugins((SplashScreenPlugin, GamePlugin, EndGamePlugin))
        .run();
}
