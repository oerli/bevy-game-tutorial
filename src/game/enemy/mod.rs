use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

pub const ENEMY_SPAWN_TIMER: f32 = 5.;
pub const ENEMY_SPEED: f32 = 10.;
pub const AMOUNT_OF_ENEMIES: usize = 1;

use resources::*;
use systems::*;

use crate::AppState;

use super::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemySpawnTimer>()
            // Enter State System
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_spawn_enemies,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameState::Running)),
            )
            // Exit Sate System
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
