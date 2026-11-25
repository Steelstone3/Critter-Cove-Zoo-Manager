use bevy::{audio::PlaybackSettings, ecs::message::Message};

#[derive(Message)]
pub struct SpawnSoundEvent {
    pub sound_path: String,
    pub playback_settings: PlaybackSettings,
}
