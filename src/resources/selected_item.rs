use bevy::ecs::system::Resource;

use crate::assets::images::animal::ZooAnimal;

#[derive(Resource)]
pub struct SelectedItem {
    pub animal: Option<ZooAnimal>,
}
