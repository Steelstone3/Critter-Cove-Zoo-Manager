use bevy::{ecs::query::QueryData, render::camera::{Camera, OrthographicProjection}, transform::components::Transform};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct CameraMutableOrthographicProjectionQuery {
    pub projection: &'static mut OrthographicProjection,
    pub camera: &'static Camera,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableCameraTransformQuery {
    pub transform: &'static mut Transform,
    pub camera: &'static Camera,
}
