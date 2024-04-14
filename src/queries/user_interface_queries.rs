use bevy::ecs::{entity::Entity, query::QueryData};

use crate::components::user_interface::SubMenu;

#[derive(QueryData)]
pub struct SubMenuEntityQuery {
    pub entity: Entity,
    pub sub_menu: &'static SubMenu,
}

// TODO Buttons query here
