use bevy::prelude::*;

use super::ASTRONAUT_SPAWN_TIMER;

#[derive(Resource)]
pub struct AstronautSpawnTimer {
    pub timer: Timer,
}

impl Default for AstronautSpawnTimer {
    fn default() -> Self {
        AstronautSpawnTimer {
            timer: Timer::from_seconds(ASTRONAUT_SPAWN_TIMER, TimerMode::Repeating),
        }
    }
}
