use bevy::prelude::*;

mod astronaut;
mod enemy;
mod player;
mod score;
mod systems;

use astronaut::AstronautPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

use crate::events::GameOver;
use crate::AppState;

use self::systems::toggle_game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<GameOver>()
            .add_plugin(AstronautPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
        // Systems
        .add_system(toggle_game.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
}