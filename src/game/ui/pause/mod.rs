mod components;
mod systems;
pub mod styles;

use bevy::prelude::*;

use systems::interactions::*;
use systems::layout::*;

use crate::game::GameState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter System
            .add_system(spawn_pause_menu.in_schedule(OnEnter(GameState::Paused)))
            // Systems
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(GameState::Paused)),
            )
            // On Exit System
            .add_system(despawn_pause_menu.in_schedule(OnExit(GameState::Paused)));
    }
}
