use super::{animation_timer::AnimationTimer, constants::TILE_SIZE};
use crate::assets::images::animal::GameAnimal;
use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Animal {
    pub source: GameAnimal,
    pub animation_timer: AnimationTimer,
    pub size: Vec2,
}

impl Animal {
    pub fn new_boar() -> Self {
        Self {
            source: GameAnimal::Boar,
            animation_timer: AnimationTimer::new(0.1, 4),
            size: Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
        }
    }
}
