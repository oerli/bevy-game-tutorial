use bevy::prelude::*;

pub mod resources;
mod systems;

use systems::*;

use crate::AppState;
use resources::*;

use super::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<HighScores>()
            // Enter State Systems
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(update_score.in_set(OnUpdate(GameState::Running)))
            .add_system(update_high_scores)
            .add_system(high_scores_updated)
            // Exit Sate Systems
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
