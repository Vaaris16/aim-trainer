use bevy::{camera_controller::free_camera::FreeCameraState, prelude::*};

use crate::game::player::player_plugin::Player;

pub fn disable_free_cam(mut player: Single<&mut FreeCameraState, With<Player>>) {
    player.enabled = false;
}

pub fn enable_free_cam(mut player: Single<&mut FreeCameraState, With<Player>>) {
    player.enabled = true;
}
