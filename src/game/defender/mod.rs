pub mod systems;
pub mod components;

use bevy::prelude::*;

use self::{components::DefenderStats, systems::move_selected_defender};

use super::mouse::MouseSelectionState;

pub struct DefenderPlugin;

impl Plugin for DefenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DefenderStats::default())
            .add_systems(Update, move_selected_defender
                .run_if(in_state(MouseSelectionState::NewDefenderSelected))
                )
            ;
    }
}
