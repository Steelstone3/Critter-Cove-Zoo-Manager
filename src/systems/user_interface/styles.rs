use bevy::{asset::AssetServer, ecs::system::Res, render::color::Color, ui::{node_bundles::{ButtonBundle, ImageBundle}, Style, UiImage, UiRect, Val}};

use crate::{assets::images::{animal::ZooAnimal, user_interface::{animal_sub_menu::AnimalSubMenu, rock_sub_menu::RockSubMenu}, world::rocks::WorldRock}, components::user_interface::{SelectAnimalButton, SelectRockButton}};

pub fn create_animal_button_bundle(animal: ZooAnimal) -> (ButtonBundle, SelectAnimalButton) {
    (
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),
                ..Default::default()
            },
            border_color: Color::DARK_GRAY.into(),
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

pub fn create_rock_button_bundle(rock: WorldRock) -> (ButtonBundle, SelectRockButton) {
    (
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),
                ..Default::default()
            },
            border_color: Color::DARK_GRAY.into(),
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