use bevy::{core_pipeline::deferred::node, prelude::*, window::PrimaryWindow};

use crate::GameState;

pub struct CrossHairPlugin;

impl Plugin for CrossHairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_crosshair);
    }
}

#[derive(Component)]
pub struct CrossHair;

pub fn spawn_crosshair(assets_server: ResMut<AssetServer>, mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            CrossHair,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(30.),
                    height: Val::Px(30.),
                    ..Default::default()
                },
                ImageNode::new(assets_server.load("crosshair.png")),
            ));
        });
}
