

use bevy::{asset::AssetServer, ecs::{system::{Commands, Res}}, ui::{node_bundles::NodeBundle}};



pub fn spawn_selection_menu(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
) {
    commands.spawn(
        NodeBundle {
        ..Default::default()
        }
    );
}

