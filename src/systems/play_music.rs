use bevy::{
    audio::{PlaybackMode, PlaybackSettings, Volume},
    ecs::{
        event::EventWriter,
        system::{Res, ResMut},
    },
    time::Time,
};

use crate::{
    components::music::Music, events::spawn_sound_event::SpawnSoundEvent,
    resources::music_timer::MusicTimer,
};

pub fn play_music(
    mut spawn_sound_event: EventWriter<SpawnSoundEvent>,
    mut music_timer: ResMut<MusicTimer>,
    time: Res<Time>,
) {
    music_timer.timer.tick(time.delta());

    if music_timer.timer.just_finished() {
        let music = Music::default();

        spawn_sound_event.send(SpawnSoundEvent {
            sound_path: music.sound_path.to_string(),
            // playback_settings: PlaybackSettings {
            //     mode: PlaybackMode::Remove,
            //     volume: Volume::new(0.5),
            //     ..Default::default()
            // },
        });
    }
}
