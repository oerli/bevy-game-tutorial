use bevy::prelude::*;

use super::ENEMY_SPAWN_TIMER;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIMER, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct Enemies {
    pub value: u32,
}

impl Default for Enemies {
    fn default() -> Self {
        Enemies {
            value: 0,
        }
    }
}
