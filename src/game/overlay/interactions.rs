use bevy::prelude::*;

use crate::game::{attacker::AttackerSpawnState, defender::{systems::spawn_blue_defender, Coins, DefenderStats}, mouse::MouseSelectionState, GameState};

use super::{components::{NewDefender1Button, StartRoundButton}, style::{DEFENDER1_BUTTON_COLOR, DEFENDER1_HOVER_BUTTON_COLOR, HOVER_START_BUTTON_COLOR, START_BUTTON_COLOR}};


pub fn interact_start_round_button(
    mut next_attacker_spawn_state: ResMut<NextState<AttackerSpawnState>>,
    attacker_spawn_state: Res<State<AttackerSpawnState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction> ,With<StartRoundButton>)>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                if *attacker_spawn_state.get() != AttackerSpawnState::Finished {
                    next_attacker_spawn_state.set(AttackerSpawnState::Spawning);
                    // change game state to running
                    next_game_state.set(GameState::Running);
                    *background_color = START_BUTTON_COLOR.into();
                }
            },
            Interaction::Hovered => {
                *background_color = HOVER_START_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = START_BUTTON_COLOR.into();
            },
        }
    }
}

// TODO: run if GameState::Running and not Already Selected
pub fn interact_new_defender1_button(
    asset_server: Res<AssetServer>,
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction> ,With<NewDefender1Button>)>,
    mut commands: Commands,
    mut defender_stats: ResMut<DefenderStats>,
    mut next_mouse_selection_state: ResMut<NextState<MouseSelectionState>>,
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                if next_mouse_selection_state.0.clone().unwrap() == MouseSelectionState::NewDefenderSelected {
                    if defender_stats.coins >= Coins(5) {
                        *bg_color = DEFENDER1_BUTTON_COLOR.into();
                        spawn_blue_defender(&asset_server,commands);
                        next_mouse_selection_state.set(MouseSelectionState::NewDefenderSelected);
                    } else {
                        *bg_color = Color::RED.into();
                    }
                }
            },
            Interaction::Hovered => {
                *bg_color = DEFENDER1_HOVER_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *bg_color = DEFENDER1_BUTTON_COLOR.into();
            },
        }
    }
}