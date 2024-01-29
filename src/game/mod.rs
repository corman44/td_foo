pub mod systems;
pub mod map;

use bevy::prelude::*;

use self::map::system::MapPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_plugins(MapPlugin);
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
    StageEnd,
}
