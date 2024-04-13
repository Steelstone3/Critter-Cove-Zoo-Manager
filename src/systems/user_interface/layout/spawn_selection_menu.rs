use bevy::{
    asset::AssetServer, ecs::system::{Commands, Res}, hierarchy::BuildChildren, render::color::Color, ui::{node_bundles::NodeBundle, Display, GridTrack, PositionType, Style, Val}
};

use crate::components::{constants::TILE_SIZE, menu::SelectionMenu};

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
            

            // Fences

            // Terrain

            // Trees

            // Rocks

            // Shelter
        });
}
