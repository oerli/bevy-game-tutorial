use bevy::prelude::*;

mod events;
mod systems;

mod game;
mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

pub const WORLD_SIZE_X: f32 = 20.;
pub const WORLD_SIZE_Z: f32 = 20.;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // Game Plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // Startup Systems
        .add_startup_system(spawn_camera)
        // Systems
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(transition_to_game_state)
        .add_system(transition_to_menu_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
