use bevy::{core_pipeline::deferred::node, prelude::*, window::PrimaryWindow};

pub struct CrossHairPlugin;

impl Plugin for CrossHairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_crosshair);
    }
}

#[derive(Component)]
pub struct CrossHair;

fn spawn_crosshair(assets_server: ResMut<AssetServer>, mut commands: Commands) {
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
