use bevy::prelude::*;
use crate::consts::*;
use crate::input::*;
use crate::player::Player;

pub fn camera_follow(
    q_player: Query<& Transform, (With<Player>, Without<Camera>)>,
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>
) {
    if let Ok(player_transform) = q_player.get_single() {
        if let Ok(mut camera_transform) = q_camera.get_single_mut() {
            camera_transform.translation = player_transform.translation;
        }
    }
}

pub fn camera_zoom(
    mut camera_event: EventReader<CameraEvent>,
    mut q_camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>
) {
    for event in camera_event.read() {
        match event {
            CameraEvent::Zoom(x) => {
                let (mut projection, _) = q_camera.single_mut();
                projection.scale -= x;
                projection.scale = projection.scale.clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX);
            }
        }
    }
}