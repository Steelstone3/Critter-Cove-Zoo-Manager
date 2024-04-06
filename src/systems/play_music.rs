use bevy::{
    audio::{PlaybackMode, PlaybackSettings, Volume},
    ecs::event::EventWriter,
};

use crate::{components::music::Music, events::spawn_sound_event::SpawnSoundEvent};

pub fn play_music(mut spawn_sound_event: EventWriter<SpawnSoundEvent>) {
    let music = Music::default();

    spawn_sound_event.send(SpawnSoundEvent {
        sound_path: music.sound_path.to_string(),
        playback_settings: PlaybackSettings {
            mode: PlaybackMode::Remove,
            volume: Volume::new(0.5),
            ..Default::default()
        },
    });
}
