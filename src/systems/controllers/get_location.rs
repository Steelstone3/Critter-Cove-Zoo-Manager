use bevy::transform::components::Transform;

use crate::queries::{
    camera_queries::CameraTransformOrthographicProjectionQueryItem, window_queries::WindowQueryItem,
};

pub fn get_cursor_location(
    transform: &mut Transform,
    position: bevy::prelude::Vec2,
    window_query: WindowQueryItem<'_>,
    camera_query: CameraTransformOrthographicProjectionQueryItem<'_>,
) {
    transform.translation.x = ((position.x - window_query.window.resolution.width() / 2.0)
        * camera_query.projection.scale)
        + camera_query.transform.translation.x;
    transform.translation.y = -((position.y - window_query.window.resolution.height() / 2.0)
        * camera_query.projection.scale)
        + camera_query.transform.translation.y;
}

#[cfg(test)]
mod get_location_should {}
