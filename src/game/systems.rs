use bevy::prelude::*;

use super::GameState;

pub fn toggle_game(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match game_state.0 {
            GameState::Running => commands.insert_resource(NextState(Some(GameState::Paused))),
            GameState::Paused => commands.insert_resource(NextState(Some(GameState::Running))),
        }
    }
}
