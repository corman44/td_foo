pub mod attacker;
pub mod defender;
pub mod map;
pub mod overlay;
pub mod player;
pub mod systems;

use bevy::prelude::*;

use self::{attacker::systems::AttackerPlugin, defender::systems::DefenderPlugin, map::system::MapPlugin, overlay::systems::OverlayPlugin, player::systems::PlayerPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_plugins((AttackerPlugin, DefenderPlugin, MapPlugin, OverlayPlugin, PlayerPlugin));
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
    StageEnd,
}
