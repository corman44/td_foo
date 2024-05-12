use bevy::prelude::*;

use super::{components::StartRoundButton, style::{get_start_button_text_style, START_BUTTON_COLOR, START_BUTTON_STYLE}};


// TODO: implement
pub fn build_game_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let _ = commands.spawn(
        (ButtonBundle {
                style: START_BUTTON_STYLE,
                background_color: START_BUTTON_COLOR.into(),
                ..default()
            },
            StartRoundButton,
        )).with_children(|parent| {
            parent.spawn(
                TextBundle {
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
}