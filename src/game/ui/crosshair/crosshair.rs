use bevy::{core_pipeline::deferred::node, prelude::*, window::PrimaryWindow};

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
                    width: Val::Px(20.),
                    height: Val::Px(20.),
                    ..Default::default()
                },
                ImageNode::new(assets_server.load("crosshair.png")),
            ));
        });
}
