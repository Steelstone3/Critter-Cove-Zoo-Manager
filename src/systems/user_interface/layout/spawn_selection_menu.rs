use bevy::{
    a11y::accesskit::TextSelection,
    asset::AssetServer,
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    render::color::Color,
    text::{Text, TextSection, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        Display, GridTrack, PositionType, Style, Val,
    },
};

use crate::components::{
    constants::TILE_SIZE, menu::SelectionMenu, user_interface::SelectAnimalButton,
};

pub fn spawn_selection_menu(mut commands: Commands, _asset_server: Res<AssetServer>) {
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
                ],
                width: Val::Px(TILE_SIZE * 2.0),
                height: Val::Px(TILE_SIZE * 6.0 * 2.0),
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
                            ..Default::default()
                        },
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    },
                    SelectAnimalButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Animals".to_string(),
                                style: TextStyle {
                                    font_size: 32.0,
                                    color: Color::BLACK.into(),
                                    ..Default::default()
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            // Fences

            // Terrain

            // Trees

            // Rocks

            // Shelter
        });
}
