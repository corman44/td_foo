pub mod component;
pub mod style;
pub mod system;

use bevy::prelude::*;

use crate::{game::{attacker::systems::move_attackers, GameState}, AppState};

use self::system::{despawn_main_menu, spawn_main_menu};

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MainMenuState {
    #[default]
    Main,
    Settings,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<MainMenuState>()
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(OnEnter(GameState::Running), move_attackers); // TODO: check if GameState is being set/updated?
    }
}