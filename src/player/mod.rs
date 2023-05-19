use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

pub const PLAYER_SPEED: f32 = 30.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(enemy_hit_player)
            .add_system(player_hit_astronaut);
    }
}
