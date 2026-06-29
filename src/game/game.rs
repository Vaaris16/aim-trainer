use bevy::prelude::*;

use crate::game::{
    level::level::LevelPlugin, player::player::PlayerPlugin, targets::target::TargetPlugin,
    ui::ui::UiPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TargetPlugin, PlayerPlugin, UiPlugin, LevelPlugin));
    }
}
