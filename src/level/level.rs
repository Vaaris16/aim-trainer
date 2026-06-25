use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_box);
    }
}

fn spawn_box(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let mesh_handle: Handle<Mesh> = asset_server.load("models/MainBox/main_box.glb#Mesh0");

    commands.spawn((
        SceneRoot(asset_server.load("models/MainBox/main_box.glb#Scene0")),
        Transform::from_xyz(0., 0., 0.),
    ));
}
