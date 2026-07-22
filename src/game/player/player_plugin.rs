use avian3d::spatial_query::{SpatialQuery, SpatialQueryFilter};
use bevy::{
    asset::saver::AssetSaver,
    camera::Hdr,
    camera_controller::free_camera::{FreeCamera, FreeCameraState},
    core_pipeline::tonemapping::Tonemapping,
    input::keyboard::NativeKeyCode,
    pbr::AtmosphereSettings,
    post_process::bloom::Bloom,
    prelude::*,
};

use crate::{
    GameState,
    game::{
        bullets::bullets::{BulletTracer, BulletsPlugins},
        targets::target::handle_hit,
        ui::score::score::Score,
    },
};

#[derive(SystemSet, PartialEq, Eq, Hash, Clone, Debug)]
struct PlayerSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, PlayerSet.run_if(in_state(GameState::Game)));
        app.add_systems(Startup, init_player)
            .add_plugins(BulletsPlugins)
            .add_systems(Update, make_ray.in_set(PlayerSet));
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Camera3d::default(),
        Hdr,
        Transform::from_xyz(2.0, 1.5, 0.0).looking_to(Vec3::X, Vec3::Y),
        Bloom::default(),
        Tonemapping::BlenderFilmic,
        AtmosphereSettings::default(),
        FreeCameraState::default(),
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
}

fn make_ray(
    mouse_input: Res<ButtonInput<MouseButton>>,
    spatial_query: SpatialQuery,
    mut commands: Commands,
    player: Query<&GlobalTransform, With<Player>>,
    assets_server: Res<AssetServer>,
    score_query: ResMut<Score>,
) {
    if !mouse_input.just_pressed(MouseButton::Right) {
        return;
    }

    if let Ok(player_trans) = player.single() {
        let origin = player_trans.translation();
        let dir = player_trans.forward();

        let ray_origin = Vec3::new(origin.x, origin.y, origin.z);
        let ray_dir = Vec3::new(dir.x, dir.y, dir.z);

        let max_toi = 50.;

        let solid = true;

        let end = ray_origin + ray_dir * 50.0;

        commands.spawn((
            WorldAssetRoot(assets_server.load("models/Bullet/bullet.glb#Scene0")),
            Transform::from_translation(ray_origin),
            BulletTracer::new(ray_origin, end, 300.0),
        ));

        if let Some(first_hit) = spatial_query.cast_ray(
            ray_origin,
            ray_dir.try_into().unwrap(),
            max_toi,
            solid,
            &SpatialQueryFilter::default(),
        ) {
            handle_hit(&mut commands, first_hit.entity, score_query);
        }
    }
}
