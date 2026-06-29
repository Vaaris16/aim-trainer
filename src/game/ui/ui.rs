use bevy::prelude::*;

use crate::{
    GameState,
    game::ui::{
        crosshair::crosshair::spawn_crosshair,
        score::score::{Score, spawn_score},
    },
};

#[derive(SystemSet, PartialEq, Eq, Hash, Clone, Debug)]
struct UiSet;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, UiSet.run_if(in_state(GameState::Game)));
        app.add_systems(OnEnter(GameState::Game), spawn_crosshair.in_set(UiSet))
            .insert_resource(Score(0))
            .add_systems(OnEnter(GameState::Game), spawn_score.in_set(UiSet));
    }
}
