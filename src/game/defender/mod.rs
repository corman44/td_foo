pub mod systems;
pub mod components;

use bevy::prelude::*;

use self::{components::DefenderStats, systems::{move_selected_defender, place_selected_defender, selected_defender_moving_highlight}};

use super::mouse::MouseSelectionState;

pub struct DefenderPlugin;

impl Plugin for DefenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DefenderStats::default())
            .add_systems(Update, (move_selected_defender, selected_defender_moving_highlight)
                .run_if(in_state(MouseSelectionState::NewDefenderSelected))
                )
            .add_systems(OnExit(MouseSelectionState::NewDefenderSelected), place_selected_defender)
            ;
    }
}
