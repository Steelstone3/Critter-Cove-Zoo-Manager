use bevy::prelude::{App, Plugin};

use crate::{assets::images::animal::ZooAnimal, resources::selected_item::SelectedItem};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedItem {
            animal: Some(ZooAnimal::Chicken),
        });
    }
}
