use bevy::{
    camera_controller::free_camera::FreeCamera, core_pipeline::tonemapping::Tonemapping,
    input::keyboard::NativeKeyCode, post_process::bloom::Bloom, prelude::*, render::view::Hdr,
};
use bevy_rapier3d::prelude::*;

use crate::{
    GameState,
    game::{
        targets::target::{Target, manage_targets},
        ui::score::score::{Score, ScoreText},
    },
};

#[derive(SystemSet, PartialEq, Eq, Hash, Clone, Debug)]
struct PlayerSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, PlayerSet.run_if(in_state(GameState::Game)));
        app.add_systems(OnEnter(GameState::Game), init_player.in_set(PlayerSet))
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
}

fn make_ray(
    mouse_input: Res<ButtonInput<MouseButton>>,
    context: ReadRapierContext,
    commands: Commands,
    player: Query<&GlobalTransform, With<Player>>,
    target_query: Query<Entity, With<Target>>,
    score_text: Query<&mut Text, With<ScoreText>>,
    score_query: ResMut<Score>,
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
            manage_targets(commands, target_query, entity, score_text, score_query);
        }
    }
}
