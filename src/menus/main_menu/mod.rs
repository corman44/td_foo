pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::AppState;

use self::systems::{despawn_main_menu, spawn_main_menu};

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
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}