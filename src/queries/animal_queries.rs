use bevy::{ecs::query::QueryData, transform::components::Transform};

use crate::components::animal::Animal;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MutableAnimalTransformQuery {
    pub transform: &'static mut Transform,
    pub animal: &'static Animal,
}
