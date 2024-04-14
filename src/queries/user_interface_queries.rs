use bevy::{
    ecs::{
        entity::Entity,
        query::{Changed, QueryData, QueryFilter},
    },
    ui::Interaction,
};

use crate::components::user_interface::{SelectAnimalButton, SelectAnimalMenuButton, SubMenu};

#[derive(QueryData)]
pub struct SubMenuEntityQuery {
    pub entity: Entity,
    pub sub_menu: &'static SubMenu,
}

#[derive(QueryData)]
pub struct SelectAnimalMenuButtonQuery {
    pub interaction: &'static Interaction,
    pub selected_animal_menu_button: &'static SelectAnimalMenuButton,
}

#[derive(QueryData)]
pub struct SelectAnimalButtonQuery {
    pub interaction: &'static Interaction,
    pub selected_animal_button: &'static SelectAnimalButton,
}

#[derive(QueryFilter)]
pub struct ButtonFilters {
    changed_interaction: Changed<Interaction>,
}
