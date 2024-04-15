use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    render::color::Color,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle},
        Display, GridTrack, PositionType, Style, UiImage, UiRect, Val,
    },
};

use crate::{
    assets::images::user_interface::main_menu::MainMenuUserInterface,
    components::{
        constants::TILE_SIZE, menu::SelectionMenu, user_interface::SelectAnimalMenuButton,
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
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
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
                        border_color: Color::DARK_GRAY.into(),
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

            // Terrain

            // Trees

            // Rocks

            // Shelter

            // Paths
        });
}
