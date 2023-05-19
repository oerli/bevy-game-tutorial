use astronaut::AstronautPlugin;
use bevy::prelude::*;

mod events;
mod systems;

mod astronaut;
mod enemy;
mod player;
mod score;

use events::*;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

pub const WORLD_SIZE_X: f32 = 20.;
pub const WORLD_SIZE_Z: f32 = 20.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugin(AstronautPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
