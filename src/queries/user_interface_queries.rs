use bevy::{
    ecs::{
        entity::Entity,
        query::{Changed, QueryData, QueryFilter},
    },
    ui::{BorderColor, Interaction},
};

use crate::components::user_interface::{
    SelectAnimalButton, SelectAnimalMenuButton, SelectRockButton, SelectRockMenuButton, SubMenu,
};

#[derive(QueryData)]
pub struct SubMenuEntityQuery {
    pub entity: Entity,
    pub sub_menu: &'static SubMenu,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectAnimalMenuButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_animal_menu_button: &'static SelectAnimalMenuButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectAnimalButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_animal_button: &'static SelectAnimalButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectRockMenuButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_rock_menu_button: &'static SelectRockMenuButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectRockButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_rock_button: &'static SelectRockButton,
}

#[derive(QueryFilter)]
pub struct ButtonFilters {
    changed_interaction: Changed<Interaction>,
}
