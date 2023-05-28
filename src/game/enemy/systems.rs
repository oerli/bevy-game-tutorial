use bevy::prelude::*;
use rand::prelude::*;

use super::{AMOUNT_OF_ENEMIES, ENEMY_SPEED};
use crate::{WORLD_SIZE_X, WORLD_SIZE_Z};

use super::components::*;
use super::resources::*;

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemies: ResMut<Enemies>,
) {
    for _ in 0..AMOUNT_OF_ENEMIES {
        let random_x = random::<f32>() * WORLD_SIZE_X;
        let random_z = random::<f32>() * WORLD_SIZE_Z;

        enemies.value += 1;

        commands
            .spawn((
                SceneBundle {
                    scene: asset_server.load("models/craft_racer.glb#Scene0"),
                    transform: Transform::from_xyz(random_x, 0., random_z),
                    ..default()
                },
                Enemy {
                    direction: Vec3::new(random::<f32>(), 0., random::<f32>()),
                },
            ))
            .with_children(|children| {
                children.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::WHITE,
                        intensity: 1000.0,
                        range: 5.0,
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 1., 0.),
                    ..default()
                });
            });
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    mut enemies: ResMut<Enemies>,
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn_recursive();
    }

    enemies.value = 0;
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut dierction_changed = false;

        let translation = transform.translation;
        if translation.x < 0. || translation.x > WORLD_SIZE_X {
            enemy.direction.x *= -1.;
            dierction_changed = true;
        }
        if translation.z < 0. || translation.z > WORLD_SIZE_Z {
            enemy.direction.z *= -1.;
            dierction_changed = true;
        }

        if dierction_changed {
            let sound_effect_1 = asset_server.load("audio/impactSoft_medium_000.ogg");
            let sound_effect_2 = asset_server.load("audio/impactSoft_medium_001.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            audio.play(sound_effect);
        }
    }
}

pub fn confine_enemy_movement(mut enemy_query: Query<&mut Transform, With<Enemy>>) {
    if let Ok(mut enemy_transform) = enemy_query.get_single_mut() {
        let mut translation = enemy_transform.translation;

        if translation.x < 0. {
            translation.x = 0.;
        } else if translation.x > WORLD_SIZE_X {
            translation.x = WORLD_SIZE_X;
        }

        if translation.z < 0. {
            translation.z = 0.;
        } else if translation.z > WORLD_SIZE_Z {
            translation.z = WORLD_SIZE_Z;
        }

        enemy_transform.translation = translation;
    }
}

pub fn tick_spawn_enemies(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    mut enemies: ResMut<Enemies>
) {
    if enemy_spawn_timer.timer.finished() {
        let random_x = random::<f32>() * WORLD_SIZE_X;
        let random_z = random::<f32>() * WORLD_SIZE_Z;

        enemies.value += 1;

        commands
            .spawn((
                SceneBundle {
                    scene: asset_server.load("models/craft_racer.glb#Scene0"),
                    transform: Transform::from_xyz(random_x, 0., random_z),
                    ..default()
                },
                Enemy {
                    direction: Vec3::new(random::<f32>(), 0., random::<f32>()),
                },
            ))
            .with_children(|children| {
                children.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::WHITE,
                        intensity: 1000.0,
                        range: 5.0,
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 1., 0.),
                    ..default()
                });
            });
    }
}
