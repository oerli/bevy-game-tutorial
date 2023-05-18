use bevy::prelude::*;
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 30.;
pub const ENEMY_SPEED: f32 = 10.;
pub const AMOUNT_OF_ENEMIES: usize = 1;
pub const AMOUNT_OF_ASTRONAUTS: usize = 4;
pub const WORLD_SIZE_X: f32 = 20.;
pub const WORLD_SIZE_Z: f32 = 20.;
pub const ASTRONAUT_SPAWN_TIMER: f32 = 1.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<AstronautSpawnTimer>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_astronauts)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(player_hit_astronaut)
        .add_system(update_score)
        .add_system(tick_spawn_astronauts)
        .add_system(spawn_astronauts_over_time)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Astronaut {}

#[derive(Component)]
pub struct Enemy {
    direction: Vec3,
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

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

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SceneBundle {
                scene: asset_server.load("models/craft_speederA.glb#Scene0"),
                transform: Transform::from_xyz(WORLD_SIZE_X / 2., 0., WORLD_SIZE_Z / 2.),
                ..default()
            },
            Player {},
        ))
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::WHITE,
                    intensity: 1000.0,
                    range: 5.0,
                    ..default()
                },
                transform: Transform::from_xyz(2.5, 2.5, 2.5),
                ..default()
            });
        });
}

pub fn spawn_camera(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.,
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(WORLD_SIZE_X * 1.5, WORLD_SIZE_Z / 2., WORLD_SIZE_Z * 1.5)
            .looking_at(
                Vec3::new(WORLD_SIZE_X / 2., -1., WORLD_SIZE_Z / 2.),
                Vec3::Y,
            ),
        ..default()
    });
}

pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..AMOUNT_OF_ENEMIES {
        let random_x = random::<f32>() * WORLD_SIZE_X;
        let random_z = random::<f32>() * WORLD_SIZE_Z;

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
                    transform: Transform::from_xyz(2.5, 2.5, 2.5),
                    ..default()
                });
            });
    }
}

pub fn spawn_astronauts(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..AMOUNT_OF_ASTRONAUTS {
        let random_x = random::<f32>() * WORLD_SIZE_X;
        let random_z = random::<f32>() * WORLD_SIZE_Z;

        commands
            .spawn((
                SceneBundle {
                    scene: asset_server.load("models/astronautA.glb#Scene0"),
                    transform: Transform::from_xyz(random_x, 0., random_z)
                        .with_rotation(Quat::from_rotation_y(random::<f32>() * 4.712389)),
                    ..default()
                },
                Astronaut {},
            ))
            .with_children(|children| {
                children.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::WHITE,
                        intensity: 1000.0,
                        range: 5.0,
                        ..default()
                    },
                    transform: Transform::from_xyz(1., 1., 1.),
                    ..default()
                });
            });
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut rotation = Vec4::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1., 0., 0.);
            rotation += Vec4::from(Quat::from_rotation_y(1.570796));
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1., 0., 0.);
            rotation += Vec4::from(Quat::from_rotation_y(4.712389));
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0., 0., -1.);
            rotation += Vec4::from(Quat::from_rotation_y(0.));
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0., 0., 1.);
            rotation += Vec4::from(Quat::from_rotation_y(3.141593));
        }

        if direction.length() > 0. {
            direction = direction.normalize();
            transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        }

        if rotation.length() > 0. {
            rotation = rotation.normalize();
            transform.rotation = Quat::from_vec4(rotation);
        }
    }
}

pub fn confine_player_movement(mut player_query: Query<&mut Transform, With<Player>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut translation = player_transform.translation;

        if translation.x < 0. {
            translation.x = 0.;
        } else if translation.x > WORLD_SIZE_X {
            translation.x = WORLD_SIZE_X;
        }

        if translation.z < 0. {
            translation.z = 0.;
        } else if translation.z > WORLD_SIZE_Z {
            translation.z = WORLD_SIZE_Z
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction * ENEMY_SPEED * time.delta_seconds();
        // transform.rotation = Quat::from_xyzw(0., enemy.direction.z, 0., 1.);
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
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
            translation.z = WORLD_SIZE_Z
        }

        enemy_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < 2. {
                let sound_effect = asset_server.load("audio/explosionCrunch_004.ogg");
                audio.play(sound_effect);

                commands.entity(player_entity).despawn_recursive();
            }
        }
    }
}

pub fn player_hit_astronaut(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut astronaut_query: Query<(Entity, &Transform), With<Astronaut>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (astronaut_entity, astronaut_transform) in astronaut_query.iter_mut() {
            let distance = astronaut_transform
                .translation
                .distance(player_transform.translation);
            if distance < 2. {
                let sound_effect = asset_server.load("audio/doorOpen_000.ogg");
                audio.play(sound_effect);
                score.value += 1;

                commands.entity(astronaut_entity).despawn_recursive();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn tick_spawn_astronauts(mut astronaut_spawn_timer: ResMut<AstronautSpawnTimer>, time: Res<Time>) {
    astronaut_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_astronauts_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    astronaut_spawn_timer: Res<AstronautSpawnTimer>,
) {
    if astronaut_spawn_timer.timer.finished() {
        let random_x = random::<f32>() * WORLD_SIZE_X;
        let random_z = random::<f32>() * WORLD_SIZE_Z;

        commands
            .spawn((
                SceneBundle {
                    scene: asset_server.load("models/astronautA.glb#Scene0"),
                    transform: Transform::from_xyz(random_x, 0., random_z)
                        .with_rotation(Quat::from_rotation_y(random::<f32>() * 4.712389)),
                    ..default()
                },
                Astronaut {},
            ))
            .with_children(|children| {
                children.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::WHITE,
                        intensity: 1000.0,
                        range: 5.0,
                        ..default()
                    },
                    transform: Transform::from_xyz(1., 1., 1.),
                    ..default()
                });
            });
    }
}
