use bevy::prelude::*;

pub mod components;
mod systems;

pub struct TerrainPlugin;

use crate::AppState;
use systems::*;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_system(spawn_terrain.in_schedule(OnEnter(AppState::Game)))
            // Systems
            // .add_systems(
            //     (
            //         terrain_movement,
            //         confine_terrain_movement.after(terrain_movement),
            //     )
            //         .in_set(OnUpdate(AppState::Game))
            //         .in_set(OnUpdate(GameState::Running)),
            // )
            // Exit State Systems
            .add_system(despawn_terrain.in_schedule(OnExit(AppState::Game)));
    }
}
