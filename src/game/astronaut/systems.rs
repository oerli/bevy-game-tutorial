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
                Astronaut {
                    rotation_speed_x: thread_rng().gen_range(-1.0..1.0),
                    rotation_speed_y: thread_rng().gen_range(-1.0..1.0),
                },
            ));
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
                Astronaut {
                    rotation_speed_x: random::<f32>(),
                    rotation_speed_y: random::<f32>(),
                },
            ));
    }
}

pub fn despawn_astronauts(mut commands: Commands, astronaut_query: Query<Entity, With<Astronaut>>) {
    for astronaut_entity in astronaut_query.iter() {
        commands.entity(astronaut_entity).despawn_recursive();
    }
}

pub fn astronaut_rotation(
    mut astronaut_query: Query<(&mut Transform, &Astronaut)>,
    time: Res<Time>,
) {
    for (mut transform, astronaut) in astronaut_query.iter_mut() {
        transform.rotate_x(astronaut.rotation_speed_x * time.delta_seconds());
        transform.rotate_y(astronaut.rotation_speed_y * time.delta_seconds());
    }
}
