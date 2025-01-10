use bevy::prelude::*;

use crate::consts::*;
use crate::input::*;

pub fn camera_zoom(
    mut camera_event: EventReader<CameraEvent>,
    mut q_camera: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for event in camera_event.read() {
        match event {
            CameraEvent::Zoom(x) => {
                let mut projection = q_camera.single_mut();
                projection.scale -= x;
                projection.scale = projection.scale.clamp(MIN_CAMERA_ZOOM, MAX_CAMERA_ZOOM);
            }
        }
    }
}