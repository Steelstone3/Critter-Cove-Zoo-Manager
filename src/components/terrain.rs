use super::constants::TILE_SIZE;
use crate::assets::images::world::terrain::WorldTerrain;
use bevy::{ecs::component::Component, math::Vec2};

const TERRAIN_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

#[derive(Component, Clone, Copy)]
pub struct Terrain {
    pub sprite_path: WorldTerrain,
    pub size: Vec2,
}

impl Terrain {
    pub fn new_grass() -> Self {
        Self {
            sprite_path: WorldTerrain::Grass1,
            size: TERRAIN_SIZE,
        }
    }
}
