use crate::{
    assets::images::{
        animal_sprites::AnimalSprite,
        world::{
            fence_sprites::FenceSprite, path_sprites::PathSprite, rock_sprites::RockSprite,
            terrain_sprites::TerrainSprite, tree_sprites::TreeSprite,
        },
    },
    systems::user_interface::interactions::spawn_menu::SpawnMenu,
};
use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct SelectedMenuItem {
    pub menu_selection: SpawnMenu,
    pub animal_selection: AnimalSprite,
    pub fence_selection: FenceSprite,
    pub terrain_selection: TerrainSprite,
    pub tree_selection: TreeSprite,
    pub rock_selection: RockSprite,
    // pub shelter_selection: Shelter,
    pub path_selection: PathSprite,
}

impl Default for SelectedMenuItem {
    fn default() -> Self {
        Self {
            menu_selection: SpawnMenu::None,
            animal_selection: AnimalSprite::None,
            fence_selection: FenceSprite::None,
            terrain_selection: TerrainSprite::None,
            tree_selection: TreeSprite::None,
            rock_selection: RockSprite::None,
            path_selection: PathSprite::None,
        }
    }
}

impl SelectedMenuItem {
    pub fn reset(&mut self) {
        self.animal_selection = AnimalSprite::None;
        self.fence_selection = FenceSprite::None;
        self.terrain_selection = TerrainSprite::None;
        self.tree_selection = TreeSprite::None;
        self.rock_selection = RockSprite::None;
        self.path_selection = PathSprite::None;
    }
}
