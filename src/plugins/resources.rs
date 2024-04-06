use bevy::prelude::{App, Plugin};

use crate::resources::selected_item::SelectedItem;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedItem::default());
    }
}
