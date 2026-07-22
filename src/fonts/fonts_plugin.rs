use bevy::prelude::*;

pub struct FontsPlugin;

impl Plugin for FontsPlugin {
    fn build(&self, app: &mut App) {
        println!("FontsPlugin build()");
        app.init_resource::<InterFonts>();
    }
}

#[allow(dead_code)]
#[derive(Resource)]
pub struct InterFonts {
    pub inter_thin: Handle<Font>,
    pub inter_medium: Handle<Font>,
    pub inter_bold: Handle<Font>,
}
impl FromWorld for InterFonts {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            inter_thin: asset_server.load("fonts/inter/static/Inter_24pt-Thin.ttf"),
            inter_medium: asset_server.load("fonts/inter/static/Inter_24pt-Medium.ttf"),
            inter_bold: asset_server.load("fonts/inter/static/Inter_24pt-Bold.ttf"),
        }
    }
}
