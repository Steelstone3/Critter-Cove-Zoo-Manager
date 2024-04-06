use bevy::prelude::{App, Plugin};

use crate::{
    assets::images::{animal::ZooAnimal, world::terrain::WorldTerrain},
    resources::selected_item::SelectedItem,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedItem {
            animal: ZooAnimal::Gorilla,
            terrain: WorldTerrain::None,
        });
    }
}
