use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle},
        Display, GridTrack, PositionType, Style, UiImage, UiRect, Val,
    },
};

use crate::{
    assets::images::user_interface::main_menu::MainMenuUserInterface,
    components::{
        constants::TILE_SIZE,
        menu::SelectionMenu,
        user_interface::{
            SelectAnimalMenuButton, SelectFenceMenuButton, SelectPathMenuButton,
            SelectRockMenuButton, SelectTerrainMenuButton, SelectTreeMenuButton,
        },
    },
};

pub fn spawn_selection_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(1.0)],
                grid_template_rows: vec![
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                ],
                width: Val::Px(TILE_SIZE * 2.0),
                height: Val::Px(TILE_SIZE * 7.0 * 2.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(0.0),
                top: Val::Percent(0.0),
                ..Default::default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
            ..Default::default()
        })
        .insert(SelectionMenu)
        .with_children(|parent| {
            // Animals
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            border: UiRect::new(
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                            ),

                            ..Default::default()
                        },
                        border_color: Color::srgb(189.0, 189.0, 189.0).into(),
                        ..Default::default()
                    },
                    SelectAnimalMenuButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load(MainMenuUserInterface::Animals.to_string()),
                        ),
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    });
                });
            // Fences
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            border: UiRect::new(
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                            ),

                            ..Default::default()
                        },
                        border_color: Color::srgb(189.0, 189.0, 189.0).into(),
                        ..Default::default()
                    },
                    SelectFenceMenuButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load(MainMenuUserInterface::Fences.to_string()),
                        ),
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    });
                });
            // Terrain
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            border: UiRect::new(
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                            ),

                            ..Default::default()
                        },
                        border_color: Color::srgb(189.0, 189.0, 189.0).into(),
                        ..Default::default()
                    },
                    SelectTerrainMenuButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load(MainMenuUserInterface::Terrain.to_string()),
                        ),
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    });
                });
            // Trees
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            border: UiRect::new(
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                            ),

                            ..Default::default()
                        },
                        border_color: Color::srgb(189.0, 189.0, 189.0).into(),
                        ..Default::default()
                    },
                    SelectTreeMenuButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load(MainMenuUserInterface::Trees.to_string()),
                        ),
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    });
                });
            // Rocks
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            border: UiRect::new(
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                            ),

                            ..Default::default()
                        },
                        border_color: Color::srgb(189.0, 189.0, 189.0).into(),
                        ..Default::default()
                    },
                    SelectRockMenuButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load(MainMenuUserInterface::Rocks.to_string()),
                        ),
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    });
                });
            // Shelter
            // Paths
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            border: UiRect::new(
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                            ),

                            ..Default::default()
                        },
                        border_color: Color::srgb(189.0, 189.0, 189.0).into(),
                        ..Default::default()
                    },
                    SelectPathMenuButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load(MainMenuUserInterface::Paths.to_string()),
                        ),
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    });
                });
        });
}
