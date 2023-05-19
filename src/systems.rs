use bevy::app::AppExit;
use bevy::prelude::*;

use crate::events::*;
use crate::{WORLD_SIZE_X, WORLD_SIZE_Z};

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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    let mut game_over_event = false;
    for event in game_over_event_reader.iter() {
        println!("Final Score: {:?}", event.score);
        game_over_event = true;
    }

    if game_over_event {
        app_exit_event_writer.send(AppExit);
    }
}
