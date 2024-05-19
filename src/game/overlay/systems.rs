use bevy::prelude::*;

use super::{components::{NewDefender1Button, StartRoundButton}, style::{get_defender_button_image_style, get_start_button_text_style, DEFENDER1_BUTTON_COLOR, DEFENDR_BUTTON_STYLE, START_BUTTON_COLOR, START_BUTTON_STYLE}};


// TODO: implement
pub fn build_game_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let _ = commands.spawn(
        (ButtonBundle {
                style: START_BUTTON_STYLE,
                background_color: START_BUTTON_COLOR.into(),
                border_color: Color::GREEN.into(),
                ..default()
            },
            StartRoundButton,
        )).with_children(|parent| {
            parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Start", 
                                get_start_button_text_style(&asset_server),
                            )
                        ],
                        ..default()
                    },
                    ..default()
                });
            }
        );
    commands.spawn((
        ButtonBundle {
            style: DEFENDR_BUTTON_STYLE,
            background_color: DEFENDER1_BUTTON_COLOR.into(),
            border_color: Color::BLACK.into(),
            ..default()
        },
        NewDefender1Button,
    )).with_children(|parent| {
        parent.spawn(ImageBundle {
            image: get_defender_button_image_style(&asset_server),
            style: Style {
                height: Val::Px(20.),
                width: Val::Px(20.),
                ..default()
            },
            ..default()
        });
    });
}
