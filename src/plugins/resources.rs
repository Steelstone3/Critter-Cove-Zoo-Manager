use bevy::prelude::{App, Plugin};

use crate::resources::tile_size::TileSize;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileSize { size: 32.0 });
    }
}
