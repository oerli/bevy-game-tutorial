use bevy::prelude::*;
use rand::prelude::*;

use super::AMOUNT_OF_ASTRONAUTS;
use crate::{WORLD_SIZE_X, WORLD_SIZE_Z};

use super::components::*;
use super::resources::*;

pub fn tick_spawn_astronauts(
    mut astronaut_spawn_timer: ResMut<AstronautSpawnTimer>,
    time: Res<Time>,
) {
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
