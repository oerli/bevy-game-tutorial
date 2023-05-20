use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::GameState;

pub const ASTRONAUT_SPAWN_TIMER: f32 = 1.;
pub const AMOUNT_OF_ASTRONAUTS: usize = 4;

pub struct AstronautPlugin;

impl Plugin for AstronautPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<AstronautSpawnTimer>()
            // Enter State Systems
            .add_system(spawn_astronauts.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    tick_spawn_astronauts,
                    spawn_astronauts_over_time,
                    astronaut_rotation,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_astronauts.in_schedule(OnExit(AppState::Game)));
    }
}
