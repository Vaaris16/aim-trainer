use bevy::prelude::*;

use crate::{
    GameState,
    game::{
        atmosphere::atmosphere_plugin::AtmospherePlugin, level::level_plugin::LevelPlugin,
        player::player_plugin::PlayerPlugin, targets::target_plugin::TargetPlugin,
        ui::ui_plugin::UiPlugin,
    },
    utilities::change_free_camera::enable_free_cam,
};

pub struct GamePlugin;

#[derive(SystemSet, Hash, PartialEq, Eq, Debug, Clone)]
struct GamePluginSet;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, GamePluginSet.run_if(in_state(GameState::Game)))
            .add_plugins((
                AtmospherePlugin,
                TargetPlugin,
                PlayerPlugin,
                UiPlugin,
                LevelPlugin,
            ))
            .add_systems(Update, enable_free_cam.in_set(GamePluginSet));
    }
}
