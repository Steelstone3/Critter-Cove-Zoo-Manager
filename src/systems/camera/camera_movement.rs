use bevy::{
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, ButtonInput},
    render::camera::Camera,
    time::Time,
    transform::components::Transform,
};

pub fn camera_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &mut Camera)>,
) {
    let Ok((mut transform, _)) = camera.get_single_mut() else {
        return;
    };

    let movement = 100.0 * time.delta_seconds();
    if input.pressed(KeyCode::KeyW) {
        transform.translation.y += movement;
    } else if input.pressed(KeyCode::KeyS) {
        transform.translation.y -= movement;
    }
    if input.pressed(KeyCode::KeyA) {
        transform.translation.x -= movement;
    } else if input.pressed(KeyCode::KeyD) {
        transform.translation.x += movement;
    }
}
