use bevy::{
    ecs::{
        entity::Entity,
        query::{Changed, QueryData, QueryFilter},
    },
    ui::{BorderColor, Interaction},
};

use crate::components::user_interface::{
    SelectAnimalButton, SelectAnimalMenuButton, SelectFenceButton, SelectFenceMenuButton,
    SelectPathButton, SelectPathMenuButton, SelectRockButton, SelectRockMenuButton,
    SelectTerrainButton, SelectTerrainMenuButton, SelectTreeButton, SelectTreeMenuButton, SubMenu,
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
pub struct SelectFenceMenuButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_fence_menu_button: &'static SelectFenceMenuButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectFenceButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_fence_button: &'static SelectFenceButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectTerrainMenuButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_terrain_menu_button: &'static SelectTerrainMenuButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectTerrainButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_terrain_button: &'static SelectTerrainButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectTreeMenuButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_tree_menu_button: &'static SelectTreeMenuButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectTreeButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_tree_button: &'static SelectTreeButton,
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

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectPathMenuButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_path_menu_button: &'static SelectPathMenuButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectPathButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub selected_path_button: &'static SelectPathButton,
}

#[derive(QueryFilter)]
pub struct ButtonFilters {
    changed_interaction: Changed<Interaction>,
}
