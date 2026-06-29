use bevy::{gizmos::grid, prelude::*, winit::UpdateMode};
use bevy_rapier3d::geometry::Collider;
use rand::{Rng, RngExt, rngs::ThreadRng};

use crate::{
    GameState,
    game::ui::score::score::{Score, ScoreText, update_score},
};

#[derive(SystemSet, PartialEq, Eq, Hash, Clone, Debug)]
struct TargetSet;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, TargetSet.run_if(in_state(GameState::Game)));
        app.add_systems(OnEnter(GameState::Game), spawn_target)
            .add_systems(Update, manage_dead_targets.in_set(TargetSet))
            .insert_resource(Grid::default());
    }
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct DeadTarget;

#[derive(Resource)]
pub struct Grid {
    grid_size: i32,
    cell_size: f32,
    max_targets: i32,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            grid_size: 5,
            cell_size: 0.8,
            max_targets: 3,
        }
    }
}

impl Grid {
    fn get_target_pos(&self, rand: &mut ThreadRng) -> Vec3 {
        let grid_pos = (Vec2::new(
            rand.random_range(0..self.grid_size) as f32,
            rand.random_range(0..self.grid_size) as f32,
        ) - Vec2::new(self.grid_size as f32 / 2.0, 0.0)
            + (Vec2::Y * 0.5))
            * self.cell_size;

        let y = grid_pos.y.clamp(0.5, 3.3);
        let z = grid_pos.x.clamp(-1.6, 1.3);

        Vec3::new(15., y, z)
    }
}

fn spawn_target(grid: Res<Grid>, asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut rand = rand::rng();
    for _ in 0..grid.max_targets {
        let pos = grid.get_target_pos(&mut rand);
        commands.spawn((
            SceneRoot(asset_server.load("models/Target/target.glb#Scene0")),
            Collider::ball(0.25),
            Target,
            Transform::from_xyz(pos.x, pos.y, pos.z),
        ));
    }
}

pub fn manage_targets(
    mut commands: Commands,
    targets: Query<Entity, With<Target>>,
    entity: Entity,
    score_text: Query<&mut Text, With<ScoreText>>,
    score_query: ResMut<Score>,
) {
    if targets.get(entity).is_ok() {
        update_score(score_text, score_query);
        commands.entity(entity).insert(DeadTarget);
    }
}

fn manage_dead_targets(
    mut commands: Commands,
    grid: Res<Grid>,
    dead_targets: Query<(Entity, &mut Transform), With<DeadTarget>>,
) {
    let mut rand = rand::rng();

    for (entity, mut transform) in dead_targets {
        let new_pos = grid.get_target_pos(&mut rand);

        if new_pos == transform.translation {
            transform.translation = grid.get_target_pos(&mut rand);
        }
        transform.translation = new_pos;

        commands.entity(entity).remove::<DeadTarget>();
    }
}
