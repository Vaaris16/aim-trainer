use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    prelude::*,
};

use bevy_rapier3d::prelude::*;

use crate::{
    crosshair::crosshair::CrossHairPlugin, level::level::LevelPlugin, player::player::PlayerPlugin,
    target::target::TargetPlugin,
};

mod crosshair;
mod level;
mod player;
mod target;

fn main() {
    App::new()
        .insert_resource(GlobalAmbientLight {
            color: Color::hsla(264.0, 0.76, 0.81, 1.0),
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
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FreeCameraPlugin)
        .add_plugins(TargetPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CrossHairPlugin)
        .add_plugins(LevelPlugin)
        .run();
}
