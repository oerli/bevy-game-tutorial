use bevy::prelude::*;

use super::GameState;

pub fn toggle_game(
    mut next_game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match game_state.0 {
            GameState::Running => next_game_state.set(GameState::Paused),
            GameState::Paused => next_game_state.set(GameState::Running),
        }
    }
}

pub fn pause_game(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Paused);
}

pub fn resume_game(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Running);
}
