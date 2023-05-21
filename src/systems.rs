use bevy::app::AppExit;
use bevy::prelude::*;

use crate::game::GameState;
use crate::events::*;
use crate::AppState;
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

pub fn transition_to_game_state(mut next_app_state: ResMut<NextState<AppState>>, keyboard_input: Res<Input<KeyCode>>, app_state: Res<State<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("You are in Game!");
        }
    }
}

pub fn transition_to_menu_state(mut next_app_state: ResMut<NextState<AppState>>, mut next_game_state: ResMut<NextState<GameState>>, keyboard_input: Res<Input<KeyCode>>, app_state: Res<State<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_game_state.set(GameState::Paused);
            println!("You are in Menu!");
        }
    }
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
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    for event in game_over_event_reader.iter() {
        println!("Final Score: {:?}", event.score);
        next_game_state.set(GameState::Paused);
        next_app_state.set(AppState::GameOver);
    }
}
