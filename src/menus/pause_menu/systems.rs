use bevy::prelude::*;

use crate::menus::style::{get_button_text_style, get_pause_text_style, NORMAL_BUTTON_COLOR, NORMAL_BUTTON_STYLE, PAUSE_MENU_STYLE};

use super::components::{PauseMenu, PauseMenuButtonAction};

pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let _ = commands.spawn(
        (NodeBundle {
            style: PAUSE_MENU_STYLE,
            ..default()
        },
        PauseMenu {},
    ))
    .with_children(|parent| {
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "PAUSED",
                            get_pause_text_style(&asset_server),
                        )
                    ],
                    ..default()
                },
                ..default()
            });
        
        parent.spawn(
            NodeBundle {
                style: NORMAL_BUTTON_STYLE,
                ..default()
            })
            .with_children(|parent| {
                for (action,text) in [
                    (PauseMenuButtonAction::Settings, "Settings"),
                    (PauseMenuButtonAction::Quit, "Quit")
                ] {
                    parent.spawn((
                        ButtonBundle {
                            style: NORMAL_BUTTON_STYLE,
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        action,
                    )).with_children(|parent| {
                        parent.spawn(
                            TextBundle {
                                text: Text {
                                    sections: vec![
                                        TextSection::new(
                                            text,
                                            get_button_text_style(&asset_server),
                                        )
                                    ],
                                    ..default()
                                },
                                ..default()
                            });
                    });
                }
            });

    });
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<PauseMenu>>
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}