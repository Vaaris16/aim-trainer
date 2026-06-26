use bevy::{camera_controller::free_camera::FreeCameraPlugin, prelude::*, window::WindowMode};

use bevy_rapier3d::prelude::*;

use crate::game::{
    level::level::LevelPlugin, player::player::PlayerPlugin, targets::target::TargetPlugin,
    ui::ui::UiPlugin,
};

mod game;

pub const MAIN_COLOR_PURPLE: Color = Color::hsla(264.0, 0.76, 0.81, 1.0);
pub const ACCENT_COLOR: Color = Color::hsla(290.0, 0.64, 0.55, 1.0);

fn main() {
    App::new()
        .insert_resource(GlobalAmbientLight {
            color: MAIN_COLOR_PURPLE,
            brightness: 50.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(FreeCameraPlugin)
        .add_plugins(TargetPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(LevelPlugin)
        .run();
}
