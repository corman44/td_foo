pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::AppState;

use self::systems::{despawn_main_menu, spawn_main_menu};

use super::interactions::{interact_with_attack_button, interact_with_defend_button, interact_with_multiplayer_button, interact_with_quit_button, interact_with_settings_button};

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
            .add_systems(Update, (interact_with_attack_button, interact_with_defend_button, interact_with_quit_button, interact_with_settings_button, interact_with_multiplayer_button).run_if(in_state(AppState::MainMenu)));
    }
}