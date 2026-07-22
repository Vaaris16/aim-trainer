use bevy::prelude::*;

use crate::GameState;

pub struct BulletsPlugins;

#[derive(SystemSet, Hash, PartialEq, Eq, Debug, Clone)]
pub struct BulletsSet;

impl Plugin for BulletsPlugins {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, BulletsSet.run_if(in_state(GameState::Game)));
        app.add_systems(Update, update_bullet.in_set(BulletsSet));
    }
}

#[derive(Component)]
pub struct BulletTracer {
    pub start_position: Vec3,
    pub end_position: Vec3,
    pub lifetime: f32,
    pub time_alive: f32,
}

impl BulletTracer {
    pub fn new(start: Vec3, end: Vec3, speed: f32) -> BulletTracer {
        BulletTracer {
            start_position: start,
            end_position: end,
            lifetime: Vec3::distance(start, end) / speed,
            time_alive: 0.,
        }
    }
}

fn update_bullet(
    mut commands: Commands,
    tracer_query: Query<(&mut BulletTracer, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut tracer, mut transform, entity) in tracer_query {
        tracer.time_alive += time.delta_secs();

        transform.translation = Vec3::lerp(
            tracer.start_position,
            tracer.end_position,
            f32::clamp(tracer.time_alive / tracer.lifetime, 0., 1.),
        );

        transform.look_at(tracer.end_position, Vec3::Y);

        if tracer.time_alive > tracer.lifetime {
            commands.entity(entity).despawn();
        }
    }
}
