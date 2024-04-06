

pub fn camera_movement( 
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &mut Camera)>
) {
    let movement = time.delta_seconds();
    if input.pressed(KeyCode::KeyW) {
        camera.0.y + movement;   
    }
    else if input.pressed(KeyCode::KeyS){
        camera.0.y - movement;
    }
}