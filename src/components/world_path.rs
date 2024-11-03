use super::path::Path;
use crate::assets::images::world::paths::WorldPath;
use bevy::prelude::Component;

#[allow(dead_code)]
#[derive(Component, Clone, Copy)]
pub struct MapPath {
    pub path: Path,
}

#[allow(dead_code)]
impl MapPath {
    pub fn new(sprite_path: WorldPath) -> Self {
        Self {
            path: Path::new(sprite_path),
        }
    }
}
