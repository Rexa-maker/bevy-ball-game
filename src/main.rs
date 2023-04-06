pub mod events;
mod game;
mod systems;
pub mod utils;

use bevy::prelude::*;
use game::ui::main_menu::MainMenuPlugin;
use game::GamePlugin;
use systems::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // States
        .add_state::<AppState>()
        // My plugins
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        // Startup systems
        .add_startup_system(spawn_camera)
        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
