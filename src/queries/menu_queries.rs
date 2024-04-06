use bevy::ecs::{entity::Entity, query::QueryData};

use crate::components::menu::Menu;

#[derive(QueryData)]
pub struct MenuEntityQuery {
    pub entity: Entity,
    pub menu: &'static Menu,
}
