use bevy::prelude::{App, Plugin};

use crate::resources::{camera_settings::CameraSettings, music_timer::MusicTimer, selected_item::SelectedMenuItem};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedMenuItem::default())
            .insert_resource(MusicTimer::default())
            .insert_resource(CameraSettings::default());
    }
}
