use bevy::prelude::*;

use crate::AppState;

use self::{interactions::interact_start_round_button, systems::build_game_overlay};

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
            ;
    }
}
