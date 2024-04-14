use bevy::{
    ecs::{
        entity::Entity,
        query::{Changed, QueryData, QueryFilter, With, Without},
    },
    ui::{BorderColor, Interaction},
};

use crate::components::user_interface::{SelectAnimalButton, SelectAnimalMenuButton, SubMenu};

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

#[derive(QueryFilter)]
pub struct SelectAnimalButtonFilters {
    with_select_animal_button: With<SelectAnimalButton>,
    without_select_animal_menu_button: Without<SelectAnimalMenuButton>,
    changed_interaction: Changed<Interaction>,
}

#[derive(QueryFilter)]
pub struct SelectAnimalMenuButtonFilters {
    with_select_animal_button: With<SelectAnimalMenuButton>,
    without_select_animal_menu_button: Without<SelectAnimalButton>,
    changed_interaction: Changed<Interaction>,
}
