use bevy::prelude::*;
use plugins::{
    game_resources::ResourcesPlugin, game_running::RunningPlugin,
    game_start::StartPlugin,
};

mod components;
mod plugins;
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
            ResourcesPlugin,
            StartPlugin,
            RunningPlugin,
        ))
        .run();
}
