use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

use crate::systems::{animate_boar::animate_sprites, play_music::play_music};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_music)
            .add_systems(Update, animate_sprites);
    }
}
