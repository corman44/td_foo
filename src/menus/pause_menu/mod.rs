use bevy::prelude::*;

use crate::{game::GameState, AppState};

use self::systems::{despawn_pause_menu, spawn_pause_menu};

pub mod components;
pub mod systems;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(GameState::Paused), despawn_pause_menu)
            .add_systems(OnExit(AppState::Game), despawn_pause_menu);
    }
}