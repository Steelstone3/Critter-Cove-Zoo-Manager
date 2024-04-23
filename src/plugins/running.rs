use crate::{
    components::game_states::GameState,
    systems::{
        animal_movement::animal_movement,
        animate_sprites::animate_sprites,
        camera::{
            camera_movement::camera_movement, camera_position_reset::camera_position_reset,
            camera_zoom_keyboard::camera_zoom_keyboard,
            camera_zoom_mouse_and_touchpad::camera_zoom_mouse_and_touchpad,
        },
        play_music::play_music,
        spawning::{spawn_animal::spawn_animal, spawn_rock::spawn_rock, spawn_tree::spawn_tree},
    },
};
use bevy::{
    app::Update,
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
    prelude::{App, Plugin},
};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_animal,
                spawn_tree,
                spawn_rock,
                play_music,
                animate_sprites.run_if(in_state(GameState::Playing)),
                animal_movement.run_if(in_state(GameState::Playing)),
                camera_zoom_keyboard,
                camera_zoom_mouse_and_touchpad,
                camera_movement,
                camera_position_reset,
            ),
        );
    }
}
