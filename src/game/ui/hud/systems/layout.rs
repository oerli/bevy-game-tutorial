use bevy::prelude::*;

use crate::game::ui::hud::components::*;
use crate::game::ui::hud::styles::*;
use crate::menu::styles::*;

pub fn spawn_hud_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hud_menu_entity = build_hud_menu(&mut commands, &asset_server);
}

pub fn despawn_hud_menu(mut commands: Commands, hud_menu_query: Query<Entity, With<HUDMenu>>) {
    if let Ok(hud_menu_entity) = hud_menu_query.get_single() {
        commands.entity(hud_menu_entity).despawn_recursive();
    }
}

fn build_hud_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_menu_entity = commands
        .spawn((
            NodeBundle {
                style: HUD_MENU_STYLE,
                ..default()
            },
            HUDMenu {},
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new("Score: ", get_text_style(&asset_server)),
                            TextSection::new("0", get_text_style(&asset_server)),
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    background_color: Color::DARK_GRAY.into(),
                    style: HUD_TEXT_STYLE,
                    ..default()
                },
                ScoreText {},
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Auto),
                    ..default()
                },
                ..default()
            });

            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new("Enemies: ", get_text_style(&asset_server)),
                            TextSection::new("0", get_text_style(&asset_server)),
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    background_color: Color::DARK_GRAY.into(),
                    style: HUD_TEXT_STYLE,
                    ..default()
                },
                EnemiesText {},
            ));
        })
        .id();

    hud_menu_entity
}
