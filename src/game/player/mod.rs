use bevy::prelude::*;

use crate::AppState;

use self::systems::print_player_health;

pub mod systems;
pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerHealth>()
            .add_systems(OnEnter(AppState::Game), print_player_health);

    }
}

#[derive(Clone, Debug, Deref, Resource)]
pub struct PlayerHealth(i32);

impl Default for PlayerHealth {
    fn default() -> Self {
        Self(100)
    }
}