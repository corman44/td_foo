
pub mod systems;
pub mod components;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::AppState;

use self::systems::{init_attacker_turns, move_attackers, red_tank_spawner, spawn_red_tank, turn_attackers};
use super::GameState;

pub struct AttackerPlugin;
 
impl Plugin for AttackerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<AttackerSpawnState>()
            .init_resource::<AttackerTurns>()
            .insert_resource(AttackerSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(OnEnter(AppState::Game), init_attacker_turns) // TODO: only run init_attacker_turns once (but after map is setup)
            .add_systems(Update, (spawn_red_tank, move_attackers, turn_attackers).run_if(in_state(GameState::Running)))
            .add_systems(Update, red_tank_spawner
                .run_if(in_state(AttackerSpawnState::Spawning))
                .run_if(in_state(GameState::Running)));
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

#[derive(Default, Component, Clone, Debug)]
pub enum Direct {
    NORTH,
    #[default]
    SOUTH,
    EAST,
    WEST
}

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum AttackerSpawnState {
    #[default]
    Idle,
    Spawning,
    Finished,
}

#[derive(Default, Clone, Debug, Resource)]
pub struct AttackerSpawnTimer(Timer);
