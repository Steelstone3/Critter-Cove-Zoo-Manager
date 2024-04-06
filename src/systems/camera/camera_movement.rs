use bevy::{
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, ButtonInput},
    time::Time,
};

use crate::queries::camera_queries::MutableCameraTransformQuery;

pub fn camera_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cameras: Query<MutableCameraTransformQuery>,
) {
    let Ok(mut camera) = cameras.get_single_mut() else {
        return;
    };

    let movement = 100.0 * time.delta_seconds();
    if input.pressed(KeyCode::KeyW) {
        camera.transform.translation.y += movement;
    } else if input.pressed(KeyCode::KeyS) {
        camera.transform.translation.y -= movement;
    }
    if input.pressed(KeyCode::KeyA) {
        camera.transform.translation.x -= movement;
    } else if input.pressed(KeyCode::KeyD) {
        camera.transform.translation.x += movement;
    }
}
