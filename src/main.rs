use bevy::{camera_controller::free_camera::FreeCameraPlugin, prelude::*};

use bevy_rapier3d::prelude::*;

use crate::game::game::GamePlugin;
mod game;
mod splashscreen;

use splashscreen::splashscreen::SplashScreenPlugin;

pub const MAIN_COLOR_PURPLE: Color = Color::hsla(264.0, 0.76, 0.81, 1.0);
pub const ACCENT_COLOR: Color = Color::hsla(249.0, 0.44, 0.58, 1.0);

#[derive(Default, States, Hash, Debug, PartialEq, Eq, Clone)]
enum GameState {
    #[default]
    SplashScreen,
    Game,
}

fn main() {
    App::new()
        .insert_resource(GlobalAmbientLight {
            color: MAIN_COLOR_PURPLE,
            brightness: 50.,
            ..Default::default()
        })
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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(FreeCameraPlugin)
        .init_state::<GameState>()
        .add_plugins((SplashScreenPlugin, GamePlugin))
        .run();
}
