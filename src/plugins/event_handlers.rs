use bevy::prelude::{App, Plugin};

pub struct EventHandlersPlugin;

impl Plugin for EventHandlersPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Update, play_music)
    }
}
