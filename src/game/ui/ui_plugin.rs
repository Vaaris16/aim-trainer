use bevy::prelude::*;

use crate::{
    GameState,
    game::ui::{
        crosshair::crosshair_plugin::CrossHairPlugin, score::score_plugin::ScorePlugins,
        timer::timer_plugin::TimerPlugin,
    },
};

#[derive(SystemSet, PartialEq, Eq, Hash, Clone, Debug)]
struct UiSet;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, UiSet.run_if(in_state(GameState::Game)));
        app.add_plugins(CrossHairPlugin)
            .add_plugins(TimerPlugin)
            .add_plugins(ScorePlugins);
    }
}
