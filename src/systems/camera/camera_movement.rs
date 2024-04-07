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

    let camera_speed = 100.0 * time.delta_seconds();
    let diagonal_camera_speed = diagonal_camera_speed(camera_speed);

    // Up and Right
    if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyD) {
        camera.transform.translation.x += diagonal_camera_speed;
        camera.transform.translation.y += diagonal_camera_speed;
    }
    // Down and Right
    else if input.pressed(KeyCode::KeyS) && input.pressed(KeyCode::KeyD) {
        camera.transform.translation.x += diagonal_camera_speed;
        camera.transform.translation.y -= diagonal_camera_speed;
    }
    // Down and Left
    else if input.pressed(KeyCode::KeyS) && input.pressed(KeyCode::KeyA) {
        camera.transform.translation.x -= diagonal_camera_speed;
        camera.transform.translation.y -= diagonal_camera_speed;
    }
    // Up and Left
    else if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyA) {
        camera.transform.translation.x -= diagonal_camera_speed;
        camera.transform.translation.y += diagonal_camera_speed;
    }
    //Up
    else if input.pressed(KeyCode::KeyW) {
        camera.transform.translation.y += camera_speed;
    }
    // Right
    else if input.pressed(KeyCode::KeyD) {
        camera.transform.translation.x += camera_speed;
    }
    // Down
    else if input.pressed(KeyCode::KeyS) {
        camera.transform.translation.y -= camera_speed;
    }
    // Left
    else if input.pressed(KeyCode::KeyA) {
        camera.transform.translation.x -= camera_speed;
    }
}

fn diagonal_camera_speed(camera_speed: f32) -> f32 {
    let diagonal_speed_squared = (camera_speed * camera_speed) + (camera_speed * camera_speed);
    let diagonal_speed = (diagonal_speed_squared).sqrt();

    camera_speed * (camera_speed / diagonal_speed)
}

#[cfg(test)]
mod camera_movement_should {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1.0, 0.70710677)]
    #[case(100.0, 70.71068)]
    #[case(141.142, 99.80246)]
    fn calculatediagonal_camera_speed(
        #[case] camera_speed: f32,
        #[case] expected_diagonal_speed: f32,
    ) {
        // Given
        let diagonal_camera_speed = diagonal_camera_speed(camera_speed);

        // Then
        assert_eq!(expected_diagonal_speed, diagonal_camera_speed);
    }
}
