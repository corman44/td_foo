use bevy::{app::AppExit, prelude::*};

use crate::{game::GameState, menus::style::{HOVER_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR}, AppState};

use super::main_menu::components::{AttackButton, DefendButton, MultiplayerButton, QuitButton, SettingsButton};

pub fn interact_with_attack_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<AttackButton>)>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
            },
            Interaction::Hovered => {
                *background_color = HOVER_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            },
        }
    }
}

pub fn interact_with_defend_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<DefendButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::Game);
                next_game_state.set(GameState::Paused);
            },
            Interaction::Hovered => {
                *background_color = HOVER_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            },
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            },
            Interaction::Hovered => {
                *background_color = HOVER_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            },
        }
    }
}

pub fn interact_with_settings_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<SettingsButton>)>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
            },
            Interaction::Hovered => {
                *background_color = HOVER_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            },
        }
    }
}

pub fn interact_with_multiplayer_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<MultiplayerButton>)>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
            },
            Interaction::Hovered => {
                *background_color = HOVER_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            },
        }
    }
}
