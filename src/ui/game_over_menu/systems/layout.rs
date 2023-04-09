use bevy::prelude::*;

use crate::game::events::GameOver;
use crate::game::score::resources::Score;
use crate::ui::game_over_menu::components::*;
use crate::ui::styles::{get_subtitle_text_style, get_title_text_style, MENU_STYLE, TITLE_STYLE};

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    for event in game_over_event_reader.iter() {
        build_game_over_menu(&mut commands, &asset_server, event.score);
    }
}

pub fn despawn_game_over_menu() {}

fn build_game_over_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    score: u32,
) -> Entity {
    let game_over_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MENU_STYLE,
                ..default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Game Over",
                        get_title_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Final Score: ".to_string() + &score.to_string(),
                        get_subtitle_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        })
        .id();

    game_over_menu_entity
}
