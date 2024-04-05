use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

use crate::systems::play_music::play_music;

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_music);
    }
}
