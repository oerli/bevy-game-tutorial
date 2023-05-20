use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

pub const ENEMY_SPAWN_TIMER: f32 = 5.;
pub const ENEMY_SPEED: f32 = 10.;
pub const AMOUNT_OF_ENEMIES: usize = 1;

use resources::*;
use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_startup_system(spawn_enemies)
            .add_system(enemy_movement)
            .add_system(update_enemy_direction)
            .add_system(confine_enemy_movement.after(enemy_movement))
            .add_system(tick_spawn_enemies)
            .add_system(spawn_enemies_over_time);
    }
}
