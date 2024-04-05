use bevy::prelude::*;
use plugins::{
    events::EventsPlugin, resources::ResourcesPlugin, running::RunningPlugin, start::StartPlugin,
};

mod assets;
mod components;
mod events;
mod plugins;
mod queries;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Critter Cove: Zoo Manager".to_string(),
                        resolution: (640.0, 480.0).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            EventsPlugin,
            ResourcesPlugin,
            StartPlugin,
            RunningPlugin,
        ))
        .run();
}
