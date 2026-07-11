use bevy::{
    light::{Atmosphere, SunDisk, atmosphere::ScatteringMedium},
    prelude::*,
};

use crate::GameState;

pub struct AtmospherePlugin;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
struct AtmosphereSet;

impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, AtmosphereSet.run_if(in_state(GameState::Game)));
        app.add_systems(OnEnter(GameState::Game), spawn_atmosphere);
    }
}

fn spawn_atmosphere(
    mut commands: Commands,
    mut scattering_mediums: ResMut<Assets<ScatteringMedium>>,
) {
    let earth = scattering_mediums.add(ScatteringMedium::earth(256, 256));

    commands.spawn((Atmosphere::earth(earth),));

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.,
            color: Color::srgb(0.7, 0.85, 1.0),
            shadow_maps_enabled: true,
            ..Default::default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.2, -0.5, 0.0)),
    ));
}
