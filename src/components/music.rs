use bevy::ecs::component::Component;

use crate::assets::sounds::music::GameMusic;

#[derive(Component)]
pub struct Music {
    pub source: GameMusic,
}

impl Default for Music {
    fn default() -> Self {
        Self {
            source: GameMusic::Grassland,
        }
    }
}
