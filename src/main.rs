use astronaut::AstronautPlugin;
use bevy::{app::AppExit, prelude::*};

mod astronaut;
mod enemy;
mod player;
mod score;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

pub const WORLD_SIZE_X: f32 = 20.;
pub const WORLD_SIZE_Z: f32 = 20.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AstronautPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_event::<GameOver>()
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

pub struct GameOver {
    pub score: u32,
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
