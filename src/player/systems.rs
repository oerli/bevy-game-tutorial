use bevy::prelude::*;

use super::components::*;
use super::PLAYER_SPEED;
use crate::astronaut::components::Astronaut;
use crate::enemy::components::Enemy;
use crate::score::resources::Score;
use crate::GameOver;
use crate::{WORLD_SIZE_X, WORLD_SIZE_Z};

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
                transform: Transform::from_xyz(0., 1., 0.),
                ..default()
            });
        });
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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    score: ResMut<Score>,
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

                game_over_event_writer.send(GameOver { score: score.value });
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
