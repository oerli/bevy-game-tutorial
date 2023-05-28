use bevy::prelude::*;

use crate::game::GameState;
use crate::AppState;

pub mod components;
pub mod styles;
mod systems;

use systems::layout::*;
use systems::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_hud_menu.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_hud_menu.in_schedule(OnExit(AppState::Game)))
            .add_system(
                update_score_in_hud_menu
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameState::Running)),
            )
            .add_system(
                update_enemies_in_hud_menu
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameState::Running)),
            );
    }
}
