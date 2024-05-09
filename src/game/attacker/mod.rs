
pub mod systems;
pub mod components;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use self::systems::{move_attackers, spawn_red_tank};
use super::GameState;

pub struct AttackerPlugin;

impl Plugin for AttackerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_red_tank, move_attackers).run_if(in_state(GameState::Running)));
    }
}

#[derive(Default, Component)]
pub struct Tank;

#[derive(Bundle, Default, LdtkEntity)]
pub struct RedTankAttacker {
    tank: Tank,
    sprite_bundle: SpriteBundle,
    direction: Direct,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
pub struct Direct {
    vec: IVec2,
}

impl Default for Direct {
    fn default() -> Self {
        Self { vec: IVec2 { x: 0, y: -1 } }
    }
}