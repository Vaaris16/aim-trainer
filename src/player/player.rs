use bevy::{
    camera_controller::free_camera::FreeCamera, core_pipeline::tonemapping::Tonemapping,
    input::keyboard::NativeKeyCode, log::tracing_subscriber::layer::Filter,
    post_process::bloom::Bloom, prelude::*, render::view::Hdr,
};
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player)
            .add_systems(Update, update_player);
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Camera3d::default(),
        Hdr,
        Transform::from_xyz(2.0, 2.0, 0.0).looking_to(Vec3::X, Vec3::Y),
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

fn update_player(
    mouse_input: Res<ButtonInput<MouseButton>>,
    context: ReadRapierContext,
    player: Query<&GlobalTransform, With<Player>>,
) {
    if !mouse_input.just_pressed(MouseButton::Right) {
        return;
    }

    if let player_trans = player.single().unwrap() {
        let origin = player_trans.translation();
        let dir = player_trans.forward();

        let ray_origin = bevy_rapier3d::parry::math::Vec3::new(origin.x, origin.y, origin.z);
        let ray_dir = bevy_rapier3d::parry::math::Vec3::new(dir.x, dir.y, dir.z);

        let max_toi = 50.;

        let solid = false;

        let filter = QueryFilter::default();

        if let Some((entity, toi)) = context
            .single()
            .unwrap()
            .cast_ray(ray_origin, ray_dir, max_toi, solid, filter)
        {
            let hit_point = origin + *dir * toi;
            println!("Middle click hit entity: {:?} at {:?}", entity, hit_point);
        }
    }
}
