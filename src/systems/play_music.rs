use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    ecs::system::Res,
    prelude::Commands,
};

use crate::components::music::Music;

pub fn play_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let music = Music::default();

    commands.spawn(AudioBundle {
        source: asset_server.load(music.source.to_string()),
        settings: PlaybackSettings {
            mode: PlaybackMode::Remove,
            volume: Volume::new(0.5),
            ..Default::default()
        },
    });
}
