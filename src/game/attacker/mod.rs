
pub mod systems;
pub mod components;

use std::default;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::AppState;

use self::systems::{init_attacker_turns, move_attackers, spawn_red_tank};
use super::GameState;

pub struct AttackerPlugin;

impl Plugin for AttackerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AttackerTurns>()
            .add_systems(OnEnter(AppState::Game), init_attacker_turns) // TODO: only run init_attacker_turns once (but after map is setup)
            .add_systems(Update, (spawn_red_tank, move_attackers).run_if(in_state(GameState::Running)));
    }
}

#[derive(Default, Component)]
pub struct Tank;

#[derive(Bundle, Default, LdtkEntity)]
pub struct RedTankAttacker {
    tank: Tank,
    sprite_bundle: SpriteBundle,
    tank_movement: TankMovement,
}

#[derive(Default, Component)]
pub struct TankMovement {
    direction: Direct,
    turns_done: i32,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct AttackerTurns {
    turn_locations: Vec<(i32,i32)>,
    direction: Vec<Direct>,
}

// #[derive(Component, Default)]
// pub struct Direction(Direct);

#[derive(Default, Component, Clone, Debug)]
pub enum Direct {
    NORTH,
    #[default]
    SOUTH,
    EAST,
    WEST
}


