use bevy::{
    camera::Hdr,
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    core_pipeline::tonemapping::Tonemapping,
    input::keyboard::NativeKeyCode,
    post_process::bloom::Bloom,
    prelude::*,
};

use crate::player::player::{Player, PlayerPlugin};

mod player;

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
        .add_plugins(FreeCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    add_container(&mut commands, asset_server);
}

fn add_crosshair(commands: &mut Commands) {
    commands
        .spawn(
            (Node {
                position_type: PositionType::Absolute,
                width: Val::Px(100.),
                height: Val::Px(100.),
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            }),
        )
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(6.),
                    height: Val::Px(6.),
                    justify_items: JustifyItems::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.0, 0.9, 1.0)), // Cyan tint matching your neon accents
            ));
        });
}

fn add_container(mut commands: &mut Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((
        WorldAssetRoot(asset_server.load("models/MainBox/main_box.glb#Scene0")),
        Transform::from_xyz(0., 0., 0.),
    ));
}
