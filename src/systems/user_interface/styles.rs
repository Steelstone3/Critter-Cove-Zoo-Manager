use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::system::Res,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle},
        Style, UiImage, UiRect, Val,
    },
};

use crate::{
    assets::images::{
        animal::ZooAnimal,
        user_interface::{
            animal_sub_menu::AnimalSubMenu, fence_sub_menu::FenceSubMenu,
            path_sub_menu::PathSubMenu, rock_sub_menu::RockSubMenu,
            terrain_sub_menu::TerrainSubMenu, tree_sub_menu::TreeSubMenu,
        },
        world::{
            fences::WorldFence, paths::WorldPath, rocks::WorldRock, terrains::WorldTerrain,
            trees::WorldTree,
        },
    },
    components::user_interface::{
        SelectAnimalButton, SelectFenceButton, SelectPathButton, SelectRockButton,
        SelectTerrainButton, SelectTreeButton,
    },
};

pub const GREY: Color = Color::srgb(189.0, 189.0, 189.0);

pub const YELLOW: Color = Color::srgb(255.0, 238.0, 88.0);

pub fn create_animal_button_bundle(animal: ZooAnimal) -> (ButtonBundle, SelectAnimalButton) {
    (
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),
                ..Default::default()
            },
            border_color: Color::srgb(189.0, 189.0, 189.0).into(),
            ..Default::default()
        },
        SelectAnimalButton { animal },
    )
}

pub fn create_animal_button_icon(
    asset_server: &Res<AssetServer>,
    animal_sub_menu: AnimalSubMenu,
) -> ImageBundle {
    ImageBundle {
        image: UiImage::new(asset_server.load(animal_sub_menu.to_string())),
        background_color: Color::WHITE.into(),
        ..Default::default()
    }
}

pub fn create_fence_button_bundle(fence: WorldFence) -> (ButtonBundle, SelectFenceButton) {
    (
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),
                ..Default::default()
            },
            border_color: Color::srgb(189.0, 189.0, 189.0).into(),
            ..Default::default()
        },
        SelectFenceButton { fence },
    )
}

pub fn create_fence_button_icon(
    asset_server: &Res<AssetServer>,
    fence_sub_menu: FenceSubMenu,
) -> ImageBundle {
    ImageBundle {
        image: UiImage::new(asset_server.load(fence_sub_menu.to_string())),
        background_color: Color::WHITE.into(),
        ..Default::default()
    }
}

pub fn create_terrain_button_bundle(terrain: WorldTerrain) -> (ButtonBundle, SelectTerrainButton) {
    (
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),
                ..Default::default()
            },
            border_color: Color::srgb(189.0, 189.0, 189.0).into(),
            ..Default::default()
        },
        SelectTerrainButton { terrain },
    )
}

pub fn create_terrain_button_icon(
    asset_server: &Res<AssetServer>,
    terrain_sub_menu: TerrainSubMenu,
) -> ImageBundle {
    ImageBundle {
        image: UiImage::new(asset_server.load(terrain_sub_menu.to_string())),
        background_color: Color::WHITE.into(),
        ..Default::default()
    }
}

pub fn create_tree_button_bundle(tree: WorldTree) -> (ButtonBundle, SelectTreeButton) {
    (
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),
                ..Default::default()
            },
            border_color: Color::srgb(189.0, 189.0, 189.0).into(),
            ..Default::default()
        },
        SelectTreeButton { tree },
    )
}

pub fn create_tree_button_icon(
    asset_server: &Res<AssetServer>,
    tree_sub_menu: TreeSubMenu,
) -> ImageBundle {
    ImageBundle {
        image: UiImage::new(asset_server.load(tree_sub_menu.to_string())),
        background_color: Color::WHITE.into(),
        ..Default::default()
    }
}

pub fn create_rock_button_bundle(rock: WorldRock) -> (ButtonBundle, SelectRockButton) {
    (
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),
                ..Default::default()
            },
            border_color: Color::srgb(189.0, 189.0, 189.0).into(),
            ..Default::default()
        },
        SelectRockButton { rock },
    )
}

pub fn create_rock_button_icon(
    asset_server: &Res<AssetServer>,
    rock_sub_menu: RockSubMenu,
) -> ImageBundle {
    ImageBundle {
        image: UiImage::new(asset_server.load(rock_sub_menu.to_string())),
        background_color: Color::WHITE.into(),
        ..Default::default()
    }
}

pub fn create_path_button_bundle(path: WorldPath) -> (ButtonBundle, SelectPathButton) {
    (
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),
                ..Default::default()
            },
            border_color: Color::srgb(189.0, 189.0, 189.0).into(),
            ..Default::default()
        },
        SelectPathButton { path },
    )
}

pub fn create_path_button_icon(
    asset_server: &Res<AssetServer>,
    path_sub_menu: PathSubMenu,
) -> ImageBundle {
    ImageBundle {
        image: UiImage::new(asset_server.load(path_sub_menu.to_string())),
        background_color: Color::WHITE.into(),
        ..Default::default()
    }
}
