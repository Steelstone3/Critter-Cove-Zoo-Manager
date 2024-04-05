use crate::assets::sounds::game_music::GameMusic;
use bevy::ecs::component::Component;
use rand::random;

#[derive(Component)]
pub struct Music {
    pub source: GameMusic,
}

impl Default for Music {
    fn default() -> Self {
        Self {
            source: GameMusic::random(),
        }
    }
}
