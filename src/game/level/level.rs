use bevy::prelude::*;

use crate::GameState;

#[derive(SystemSet, PartialEq, Eq, Hash, Clone, Debug)]
struct LevelSet;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, LevelSet.run_if(in_state(GameState::Game)));
        app.add_systems(Startup, spawn_box);
    }
}

fn spawn_box(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        WorldAssetRoot(asset_server.load("models/MainBox/main_box.glb#Scene0")),
        Transform::from_xyz(0., 0., 0.),
    ));
}
