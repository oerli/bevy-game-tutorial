use bevy::prelude::*;

use super::components::*;

use crate::{WORLD_SIZE_X, WORLD_SIZE_Z};

pub fn spawn_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    for x in 0..WORLD_SIZE_X as i32 + 1 {
        for z in 0..WORLD_SIZE_Z as i32 + 1 {
            let terrain_scene = if x == 0 && z == 0 {
                SceneBundle {
                    scene: asset_server.load("models/platform_cornerOpen.glb#Scene0"),
                    transform: Transform::from_xyz(x as f32, -0.4, z as f32)
                        .with_rotation(Quat::from_rotation_y(3.14)),
                    ..default()
                }
            } else if x == 0 && z == WORLD_SIZE_Z as i32 {
                SceneBundle {
                    scene: asset_server.load("models/platform_cornerOpen.glb#Scene0"),
                    transform: Transform::from_xyz(x as f32, -0.4, z as f32)
                        .with_rotation(Quat::from_rotation_y(4.71)),
                    ..default()
                }
            } else if x == WORLD_SIZE_X as i32 && z == 0 {
                SceneBundle {
                    scene: asset_server.load("models/platform_cornerOpen.glb#Scene0"),
                    transform: Transform::from_xyz(x as f32, -0.4, z as f32)
                        .with_rotation(Quat::from_rotation_y(1.57)),
                    ..default()
                }
            } else if x == WORLD_SIZE_X as i32 && z == WORLD_SIZE_Z as i32 {
                SceneBundle {
                    scene: asset_server.load("models/platform_cornerOpen.glb#Scene0"),
                    transform: Transform::from_xyz(x as f32, -0.4, z as f32),
                    ..default()
                }
            } else if x == 0 {
                SceneBundle {
                    scene: asset_server.load("models/platform_side.glb#Scene0"),
                    transform: Transform::from_xyz(x as f32, -0.4, z as f32)
                        .with_rotation(Quat::from_rotation_y(4.71)),
                    ..default()
                }
            } else if z == 0 {
                SceneBundle {
                    scene: asset_server.load("models/platform_side.glb#Scene0"),
                    transform: Transform::from_xyz(x as f32, -0.4, z as f32)
                        .with_rotation(Quat::from_rotation_y(3.14)),
                    ..default()
                }
            } else if x == WORLD_SIZE_X as i32 {
                SceneBundle {
                    scene: asset_server.load("models/platform_side.glb#Scene0"),
                    transform: Transform::from_xyz(x as f32, -0.4, z as f32)
                        .with_rotation(Quat::from_rotation_y(1.57)),
                    ..default()
                }
            } else if z == WORLD_SIZE_Z as i32 {
                SceneBundle {
                    scene: asset_server.load("models/platform_side.glb#Scene0"),
                    transform: Transform::from_xyz(x as f32, -0.4, z as f32),
                    ..default()
                }
            } else {
                SceneBundle {
                    scene: asset_server.load("models/platform_center.glb#Scene0"),
                    transform: Transform::from_xyz(x as f32, -0.4, z as f32),
                    ..default()
                }
            };

            commands.spawn(terrain_scene);
        }
    }
}

pub fn despawn_terrain(mut commands: Commands, terrain_query: Query<Entity, With<Terrain>>) {
    for terrain_entity in terrain_query.iter() {
        commands.entity(terrain_entity).despawn_recursive();
    }
}
