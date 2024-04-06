use bevy::ecs::{entity::Entity, query::QueryData};

use crate::components::menu::UserInterface;

#[derive(QueryData)]
pub struct UserInterfaceEntityQuery {
    pub entity: Entity,
    pub user_interface: &'static UserInterface,
}