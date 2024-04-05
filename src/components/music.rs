use crate::assets::sounds::game_music::GameMusic;
use bevy::ecs::component::Component;
use rand::random;

#[derive(Component)]
pub struct Music {
    pub sound_path: GameMusic,
}

impl Default for Music {
    fn default() -> Self {
        Self {
            sound_path: random(),
        }
    }
}
