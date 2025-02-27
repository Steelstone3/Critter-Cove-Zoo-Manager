use crate::events::spawn_sound_event::SpawnSoundEvent;
use bevy::{
    asset::AssetServer,
    audio::{AudioPlayer, AudioSource},
    ecs::{
        event::EventReader,
        system::{Commands, Res},
    },
};

pub fn spawn_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_sound_events: EventReader<SpawnSoundEvent>,
) {
    for spawn_sound_event in spawn_sound_events.read() {
        let source = asset_server.load(&spawn_sound_event.sound_path);
        
        // TODO AH Need to work out how to add settings
        // let settings = spawn_sound_event.playback_settings;

        commands.spawn(AudioPlayer::<AudioSource>(source));
    }
}
