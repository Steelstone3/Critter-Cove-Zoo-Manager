use crate::assets::images::world::{rocks::WorldRock, tree::WorldTree};
use bevy::{ecs::component::Component, math::Vec2};

use super::constants::TILE_SIZE;

#[derive(Component)]
pub struct Tree {
    pub sprite_path: WorldTree,
    pub size: Vec2,
}

impl Tree {
    pub fn new(sprite_path: WorldTree) -> Self {
        Self {
            sprite_path,
            size: Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
        }
    }
}
