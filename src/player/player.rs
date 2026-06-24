use bevy::{
    camera::Hdr, camera_controller::free_camera::FreeCamera,
    core_pipeline::tonemapping::Tonemapping, input::keyboard::NativeKeyCode,
    post_process::bloom::Bloom, prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player);
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Camera3d::default(),
        Transform::from_xyz(2.0, 2.0, 0.0).looking_to(Vec3::X, Vec3::Y),
        Hdr,
        Bloom::default(),
        Tonemapping::TonyMcMapface,
        FreeCamera {
            key_forward: KeyCode::Unidentified(NativeKeyCode::Unidentified),
            key_up: KeyCode::Unidentified(NativeKeyCode::Unidentified),
            key_down: KeyCode::Unidentified(NativeKeyCode::Unidentified),
            key_left: KeyCode::Unidentified(NativeKeyCode::Unidentified),
            key_right: KeyCode::Unidentified(NativeKeyCode::Unidentified),
            key_back: KeyCode::Unidentified(NativeKeyCode::Unidentified),
            ..Default::default()
        },
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 100000.,
            ..Default::default()
        },
        Transform::from_xyz(2., 2., 0.).looking_to(Vec3::X, Vec3::Y),
    ));
}
