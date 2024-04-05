use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    ecs::system::{Query, Res},
    prelude::Commands,
};

use crate::{components::music::Music, queries::music_queries::MusicQuery};

pub fn play_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    musics: Query<MusicQuery>,
) {
    let Ok(_) = musics.get_single() else {
        let music = Music::default();

        commands
            .spawn(AudioBundle {
                source: asset_server.load(music.source.to_string()),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    volume: Volume::new(0.5),
                    ..Default::default()
                },
            })
            .insert(music);

        return;
    };
}
