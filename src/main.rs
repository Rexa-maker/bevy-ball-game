mod game;
mod systems;
mod ui;

use bevy::prelude::*;
use game::GamePlugin;
use systems::*;
use ui::{main_menu::MainMenuPlugin, UIPlugin};

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // States
        .add_state::<AppState>()
        // My plugins
        .add_plugin(GamePlugin)
        .add_plugin(UIPlugin)
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
