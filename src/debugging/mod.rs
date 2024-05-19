pub mod systems;
pub mod components;

use bevy::prelude::*;

use crate::game::defender::components::DefenderStats;

use self::systems::{debug_timer, mouse_button_input, print_attacker_locations, print_defender_stats, print_state_change};

pub struct DebuggingPlugin;

impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DebugTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(DebugCounter(0))
            .add_systems(Update, (print_state_change, debug_timer))
            .add_systems(Update, (
                print_defender_stats,
                // print_attacker_locations
            ).run_if(
                resource_changed::<DefenderStats>()
                // .and_then(not(resource_added::<DefenderStats>()))
                // resource_exists::<DefenderStats>()
                // .and_then(resource_changed::<DefenderStats>())
                // .or_else(resource_changed::<DebugCounter>())
            ));
    }
}

#[derive(Resource)]
pub struct DebugTimer(Timer);

#[derive(Resource)]
pub struct DebugCounter(i32);
