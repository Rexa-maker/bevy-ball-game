use crate::ui::hud::components::*;
use crate::ui::styles::*;
use bevy::prelude::*;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}

fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    size: Size::new(Val::Percent(100.0), Val::Px(64.0)),
                    ..default()
                },
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle { ..default() })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: IMAGE_STYLE,
                                image: asset_server.load("sprites/star.png").into(),
                                ..default()
                            });
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Score goes here",
                                        get_title_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Right,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    hud_entity
}
