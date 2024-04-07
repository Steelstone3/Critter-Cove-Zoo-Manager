use bevy::{
    ecs::system::Resource,
    time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct MusicTimer {
    pub timer: Timer,
}

impl Default for MusicTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(65.0, TimerMode::Repeating),
        }
    }
}
