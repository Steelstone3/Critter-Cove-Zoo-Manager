use bevy::{
    ecs::{
        schedule::{NextState, State},
        system::{Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
};

use crate::components::game_states::GameState;

pub fn initial_state(mut next_game_state: ResMut<NextState<GameState>>) {
    // TODO move this to main menu
    next_game_state.set(GameState::Playing);
}

pub fn toggle_pause(
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut input: ResMut<ButtonInput<KeyCode>>,
) {
    if !input.clear_just_pressed(KeyCode::KeyP) {
        return;
    }

    let game_state = game_state.get();
    if game_state == &GameState::Playing {
        next_game_state.set(GameState::Paused);
    } else {
        next_game_state.set(GameState::Playing);
    }
}
