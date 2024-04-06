use bevy::prelude::{App, Plugin};

use crate::resources::selected_item::SelectedMenuItem;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedMenuItem::default());
    }
}
