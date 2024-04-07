use crate::{
    queries::camera_queries::CameraMutableOrthographicProjectionQuery,
    resources::camera_settings::CameraSettings,
};
use bevy::{
    ecs::{
        event::EventReader,
        system::{Query, ResMut},
    },
    input::{
        mouse::{MouseButton, MouseButtonInput, MouseWheel},
        ButtonState,
    },
};
use float_lerp::lerp;

pub fn camera_zoom_mouse_and_touchpad(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut cameras: Query<CameraMutableOrthographicProjectionQuery>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    let Ok(mut camera) = cameras.get_single_mut() else {
        return;
    };

    for mouse_wheel_event in mouse_wheel_events.read() {
        if mouse_wheel_event.y < 0.0 {
            camera_settings.current_zoom = (camera_settings.current_zoom
                * camera_settings.zoom_in
                * camera_settings.zoom_speed)
                .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
        } else if mouse_wheel_event.y > 0.0 {
            camera_settings.current_zoom = (camera_settings.current_zoom
                * camera_settings.zoom_out
                / camera_settings.zoom_speed)
                .clamp(camera_settings.minimum_zoom, camera_settings.maximum_zoom);
        }
    }

    for mouse_button_event in mouse_button_events.read() {
        if mouse_button_event.button != MouseButton::Middle {
            return;
        }
        if mouse_button_event.state != ButtonState::Pressed {
            return;
        }

        camera_settings.current_zoom = 1.0;
    }

    camera.projection.scale = lerp(camera.projection.scale, camera_settings.current_zoom, 0.05);
}
