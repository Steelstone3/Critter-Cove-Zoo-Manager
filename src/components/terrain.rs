use super::constants::TILE_SIZE;
use crate::assets::images::world::terrains::WorldTerrain;
use bevy::{ecs::component::Component, math::Vec2};

const TERRAIN_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

#[derive(Component, Clone, Copy)]
pub struct Terrain {
    pub sprite_path: WorldTerrain,
    pub size: Vec2,
    pub z_index: f32,
}

impl Terrain {
    pub fn new(sprite_path: WorldTerrain) -> Self {
        Self {
            sprite_path,
            size: TERRAIN_SIZE,
            z_index: 0.0,
        }
    }

    pub fn new_player(sprite_path: WorldTerrain) -> Self {
        Self {
            sprite_path,
            size: TERRAIN_SIZE,
            z_index: 1.0,
        }
    }
}
