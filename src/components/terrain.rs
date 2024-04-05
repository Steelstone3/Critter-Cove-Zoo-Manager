use super::constants::TILE_SIZE;
use crate::assets::images::world::terrain::WorldTerrain;
use bevy::{ecs::component::Component, math::Vec2};

const SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

#[derive(Component)]
pub struct Terrain {
    pub sprite_path: WorldTerrain,
    pub size: Vec2,
}

impl Terrain {
    pub fn new_grass() -> Self {
        Self {
            sprite_path: WorldTerrain::Grass,
            size: SIZE,
        }
    }
}
