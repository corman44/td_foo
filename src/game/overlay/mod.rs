use bevy::prelude::*;

use crate::AppState;

use self::{interactions::{interact_new_defender1_button, interact_start_round_button}, systems::build_game_overlay};

use super::GameState;

pub mod systems;
pub mod components;
pub mod style;
pub mod interactions;


pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), build_game_overlay)
            .add_systems(Update, interact_start_round_button.run_if(in_state(AppState::Game)))
            .add_systems(Update, interact_new_defender1_button
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running))
            )
            ;
    }
}
