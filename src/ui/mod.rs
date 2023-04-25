mod game_over_menu;
mod hud;
pub mod main_menu;
mod pause_menu;
mod styles;

use bevy::prelude::*;
use styles::*;

use self::{
    game_over_menu::GameOverMenuPlugin, hud::HUDPlugin, main_menu::MainMenuPlugin,
    pause_menu::PauseMenuPlugin,
};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugin(MainMenuPlugin)
            .add_plugin(GameOverMenuPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_plugin(HUDPlugin);
    }
}

pub fn interact_with_button_changes_color(
    interaction: &Interaction,
    background_color: &mut Mut<BackgroundColor>,
) {
    match *interaction {
        Interaction::Clicked => {
            **background_color = PRESSED_BUTTON_COLOR.into();
        }
        Interaction::Hovered => {
            **background_color = HOVERED_BUTTON_COLOR.into();
        }
        Interaction::None => {
            **background_color = NORMAL_BUTTON_COLOR.into();
        }
    }
}
