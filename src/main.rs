use bevy::prelude::*;
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    direction: Vec3,
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SceneBundle {
                scene: asset_server.load("models/craft_speederA.glb#Scene0"),
                transform: Transform::from_xyz(10., 0., 10.),
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
        transform: Transform::from_xyz(20., 20., 20.).looking_at(Vec3::new(10., 0., 10.), Vec3::Y),
        ..default()
    });
}

pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..3 {
        let random_x = random::<f32>() * 20.;
        let random_z = random::<f32>() * 20.;

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

pub const PLAYER_SPEED: f32 = 50.;

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
        } else if translation.x > 20. {
            translation.x = 20.;
        }

        if translation.z < 0. {
            translation.z = 0.;
        } else if translation.z > 20. {
            translation.z = 20.
        }

        player_transform.translation = translation;
    }
}

pub const ENEMY_SPEED: f32 = 10.;

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(mut enemy_query: Query<(&mut Transform, &mut Enemy)>) {
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < 0. || translation.x > 20. {
            enemy.direction.x *= -1.;
        }
        if translation.z < 0. || translation.z > 20. {
            enemy.direction.z *= -1.;
        }
    }
}

pub fn confine_enemy_movement(mut enemy_query: Query<&mut Transform, With<Enemy>>) {
    if let Ok(mut enemy_transform) = enemy_query.get_single_mut() {
        let mut translation = enemy_transform.translation;

        if translation.x < 0. {
            translation.x = 0.;
        } else if translation.x > 20. {
            translation.x = 20.;
        }

        if translation.z < 0. {
            translation.z = 0.;
        } else if translation.z > 20. {
            translation.z = 20.
        }

        enemy_transform.translation = translation;
    }
}
