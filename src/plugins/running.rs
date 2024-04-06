use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

use crate::systems::animate_sprites::animate_sprites;

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprites);
    }
}
