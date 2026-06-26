use bevy::prelude::*;

use crate::game::ui::{
    crosshair::crosshair::spawn_crosshair,
    score::score::{Score, spawn_score},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_crosshair)
            .insert_resource(Score(0))
            .add_systems(Startup, spawn_score);
    }
}
