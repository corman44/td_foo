pub mod attacker;
pub mod defender;
pub mod map;
pub mod overlay;
pub mod player;
pub mod systems;
pub mod mouse;

use bevy::prelude::*;

use crate::AppState;

use self::{
    attacker::AttackerPlugin, defender::systems::DefenderPlugin, map::MapPlugin, mouse::MousePlugin, overlay::OverlayPlugin, player::PlayerPlugin, systems::pause_game
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_plugins((AttackerPlugin, DefenderPlugin, MapPlugin, OverlayPlugin, PlayerPlugin, MousePlugin))
            .add_systems(Update, pause_game.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
    StageEnd,
}
