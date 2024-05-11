use bevy::prelude::*;

use crate::menus::style::{get_button_text_style, get_title_text_style, IMAGE_STYLE, MAIN_MENU_STYLE, NORMAL_BUTTON_COLOR, NORMAL_BUTTON_STYLE, TITLE_STYLE};

use super::components::{AttackButton, DefendButton, MainMenu, MultiplayerButton, QuitButton, SettingsButton};


pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let _ = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entry = commands
        .spawn(
            (NodeBundle {
                style: MAIN_MENU_STYLE,
                // border_color: Color::RED.into(),
                ..default()
            },

            MainMenu {},
        ))
        .with_children(|parent| {
            // --- Title Section
            parent.spawn(
                NodeBundle {
                    style: TITLE_STYLE,
                    // border_color: Color::RED.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("kenney_top-down-tanks-redux/PNG/Default size/tank_huge.png").into(),
                        ..default()
                    });

                    parent.spawn(
                        TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "TD_FOO",
                                        get_title_text_style(&asset_server),
                                    )
                                ],
                                ..default()
                            },
                            ..default()
                        });
                    
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("kenney_top-down-tanks-redux/PNG/Default size/tank_huge.png").into(),
                        ..default()
                    });
                });
            
            // --- First Button Row Section ---
            parent.spawn(
                NodeBundle {
                    style: NORMAL_BUTTON_STYLE,
                    // border_color: Color::RED.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: NORMAL_BUTTON_STYLE,
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        DefendButton,
                    )).with_children(|parent| {
                        parent.spawn(
                            TextBundle {
                                text: Text {
                                    sections: vec![
                                        TextSection::new(
                                            "Defend",
                                            get_button_text_style(&asset_server),
                                        )
                                    ],
                                    ..default()
                                },
                                ..default()
                            });
                    });
                    parent.spawn((
                        ButtonBundle {
                            style: NORMAL_BUTTON_STYLE,
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        AttackButton,
                    )).with_children(|parent| {
                        parent.spawn(
                            TextBundle {
                                text: Text {
                                    sections: vec![
                                        TextSection::new(
                                            "Attack",
                                            get_button_text_style(&asset_server),
                                        )
                                    ],
                                    ..default()
                                },
                                ..default()
                            });
                    });
                    parent.spawn((
                        ButtonBundle {
                            style: NORMAL_BUTTON_STYLE,
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        MultiplayerButton,
                    )).with_children(|parent| {
                        parent.spawn(
                            TextBundle {
                                text: Text {
                                    sections: vec![
                                        TextSection::new(
                                            "Multi",
                                            get_button_text_style(&asset_server),
                                        )
                                    ],
                                    ..default()
                                },
                                ..default()
                            });
                    });
            });

            // --- Second Button Row Section ---
            parent.spawn(
                NodeBundle {
                    // border_color: Color::RED.into(),
                    style: NORMAL_BUTTON_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: NORMAL_BUTTON_STYLE,
                            // border_color: Color::RED.into(),
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        SettingsButton ,
                    )).with_children(|parent| {
                        parent.spawn(
                            TextBundle {
                                text: Text {
                                    sections: vec![
                                        TextSection::new(
                                            "Settings",
                                            get_button_text_style(&asset_server),
                                        )
                                    ],
                                    ..default()
                                },
                                ..default()
                            });
                    });
                    parent.spawn((
                        ButtonBundle {
                            style: NORMAL_BUTTON_STYLE,
                            // border_color: Color::RED.into(),
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        QuitButton,
                    )).with_children(|parent| {
                        parent.spawn(
                            TextBundle {
                                text: Text {
                                    sections: vec![
                                        TextSection::new(
                                            "Quit",
                                            get_button_text_style(&asset_server),
                                        )
                                    ],
                                    ..default()
                                },
                                ..default()
                            });
                    });
                });
            })
        .id();
        main_menu_entry
}