use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

pub const ASTRONAUT_SPAWN_TIMER: f32 = 1.;
pub const AMOUNT_OF_ASTRONAUTS: usize = 4;

pub struct AstronautPlugin;

impl Plugin for AstronautPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AstronautSpawnTimer>()
            .add_startup_system(spawn_astronauts)
            .add_system(tick_spawn_astronauts)
            .add_system(spawn_astronauts_over_time);
    }
}
