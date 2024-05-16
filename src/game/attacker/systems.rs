use std::{collections::HashSet, f32::consts::PI};

use bevy::{prelude::*, transform::commands};
use bevy_ecs_ldtk::prelude::*;

use crate::game::{attacker::TankMovement, map::AttackerArea, player::PlayerHealth};

use super::{AttackerSpawnState, AttackerSpawnTimer, AttackerTurns, Direct, RedTankAttacker, Tank};

const NORTH_GC: GridCoords = GridCoords{ x:0, y:1 };
const EAST_GC: GridCoords = GridCoords{ x:1, y:0 };
const SOUTH_GC: GridCoords = GridCoords{ x:0, y:-1 };
const WEST_GC: GridCoords = GridCoords{ x:-1, y:0 };
const SPEED: f32 = 100.0;

pub fn move_attackers(
    mut attacker_query: Query<(&mut TankMovement, &mut Transform)>,
    attacker_turns: Res<AttackerTurns>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    // TODO: check if new transform is past turning point, if so apply direciton until turn, then apply remaining in new direction
    for (mut tank_movement, mut tank_transform) in attacker_query.iter_mut() {
        let turn_index = tank_movement.turns_done as usize;
        match tank_movement.direction {
            Direct::NORTH => {
                // info!("tank NORTH");
                if let Some(next_max) = attacker_turns.turn_locations.get(turn_index) {
                    if tank_transform.translation.y + delta * SPEED > next_max.1 as f32 {
                        // info!("Turn found at: {:?}", tank_transform.translation);
                        tank_movement.direction = attacker_turns.direction.get(turn_index).unwrap().clone();
                        tank_transform.translation.y = attacker_turns.turn_locations.get(turn_index).unwrap().1 as f32;
                        tank_movement.turns_done += 1;
                    } else {
                        tank_transform.translation.y += delta * SPEED;
                    }
                } else {
                    tank_transform.translation.y += delta * SPEED;
                }
            },
            Direct::SOUTH => {
                // info!("tank SOUTH");
                if let Some(next_max) = attacker_turns.turn_locations.get(turn_index) {
                    if tank_transform.translation.y + (delta * SPEED) < next_max.1 as f32 {
                        // info!("Turn found at: {:?}", tank_transform.translation);
                        tank_movement.direction = attacker_turns.direction.get(turn_index).unwrap().clone();
                        tank_transform.translation.y = attacker_turns.turn_locations.get(turn_index).unwrap().1 as f32;
                        tank_movement.turns_done += 1;
                    } else {
                        tank_transform.translation.y -= delta * SPEED;
                    }
                }
                else {
                    tank_transform.translation.y -= delta * SPEED;
                }
            },
            Direct::EAST => {
                // info!("tank EAST");
                if let Some(next_max) = attacker_turns.turn_locations.get(turn_index) {
                    if tank_transform.translation.x + delta * SPEED > next_max.0 as f32 {
                        // info!("Turn found at: {:?}", tank_transform.translation);
                        tank_movement.direction = attacker_turns.direction.get(turn_index).unwrap().clone();
                        tank_transform.translation.x = attacker_turns.turn_locations.get(turn_index).unwrap().0 as f32;
                        tank_movement.turns_done += 1;
                    } else {
                        tank_transform.translation.x += delta * SPEED;
                    }
                }
                else {
                    tank_transform.translation.x += delta * SPEED;
                }
            },
            Direct::WEST => {
                // info!("tank WEST");
                if let Some(next_max) = attacker_turns.turn_locations.get(turn_index) {
                    if tank_transform.translation.x + delta * SPEED < next_max.0 as f32 {
                        // info!("Turn found at: {:?}", tank_transform.translation);
                        tank_movement.direction = attacker_turns.direction.get(turn_index).unwrap().clone();
                        tank_transform.translation.x = attacker_turns.turn_locations.get(turn_index).unwrap().0 as f32;
                        tank_movement.turns_done += 1;
                    } else {
                        tank_transform.translation.x -= delta * SPEED;
                    }
                } else {
                    tank_transform.translation.x -= delta * SPEED;
                }
            },
        }
    }
}

pub fn turn_attackers(
    mut attacker_query: Query<(&mut TankMovement, &mut Transform)>,
) {
    // TODO: update query to only return if Direct Changed...
    for (tank_movement, mut tank_trans) in attacker_query.iter_mut() {
        match tank_movement.direction {
            Direct::NORTH => tank_trans.rotation = Quat::from_rotation_z(0.),
            Direct::SOUTH => tank_trans.rotation = Quat::from_rotation_z(PI),
            Direct::EAST => tank_trans.rotation = Quat::from_rotation_z(-PI/2.),
            Direct::WEST => tank_trans.rotation = Quat::from_rotation_z(PI/2.),
        }
    }
}

pub fn spawn_red_tank(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::S) {
        let texture = asset_server.load(r"kenney_top-down-tanks-redux\PNG\Default size\tank_red_cj.png");
        let id = commands.spawn(RedTankAttacker {
            sprite_bundle: SpriteBundle {
                texture: texture,
                transform: Transform { translation: Vec3::new(480., 992., 6.), ..default() },
                ..default()
            },
            tank_movement: TankMovement {
                turns_done: 0,
                direction: Direct::SOUTH,
            },
            ..default()
        }).id();
        // info!("RedTankAttacker spawned: id={}",id.index());
    }
}

pub fn red_tank_spawner(
    asset_server: Res<AssetServer>,
    mut attacker_spawn_timer: ResMut<AttackerSpawnTimer>,
    mut commands: Commands,
    attacker_query: Query<(&mut TankMovement, &mut Transform)>,
    mut next_attacker_spawn_state: ResMut<NextState<AttackerSpawnState>>,
    time: Res<Time>,
) {
    if attacker_spawn_timer.0.tick(time.delta()).just_finished() && attacker_query.iter().count() < 5 {
        let texture = asset_server.load(r"kenney_top-down-tanks-redux\PNG\Default size\tank_red_cj.png");
        let id = commands.spawn(RedTankAttacker {
            sprite_bundle: SpriteBundle {
                texture,
                transform: Transform { translation: Vec3::new(480., 992., 6.), ..default() },
                ..default()
            },
            tank_movement: TankMovement {
                turns_done: 0,
                direction: Direct::SOUTH,
            },
            ..default()
        }).id();
        attacker_spawn_timer.0.reset();
    } else if attacker_query.iter().count() >= 5 {
        next_attacker_spawn_state.set(AttackerSpawnState::Finished);
    }
    // info!("attacker_spawner_timer = {:?}", attacker_spawn_timer);
    // info!("attacker_count = {:?}", attacker_query.iter().count());
}

pub fn init_attacker_turns(
    mut turns_res: ResMut<AttackerTurns>,
    attack_tiles_query: Query<&GridCoords, (With<AttackerArea>, Without<Tank>)>,
) {
    let attack_tiles_hs: HashSet<GridCoords> = attack_tiles_query.iter().copied().collect();
    let mut attacker_turns = AttackerTurns { turn_locations: vec![], ..default()};
    let mut turn_locations_gc: Vec<GridCoords> = vec![];
    let mut curr_pos = GridCoords {x: 7, y: 15};
    let mut curr_dir = Direct::SOUTH;
    let mut done: bool = false;

    // info!("attack_tiles: {:?}", attack_tiles_hs.clone());
    while(!done) {
        // if dir south and found south, update curr_pos and continue.
        // otherwise, turn found and need to push the turn info to attacker_turns
        match curr_dir {
            Direct::NORTH => {
                // info!("matched NORTH");
                if attack_tiles_hs.contains(&(curr_pos + NORTH_GC)) {
                    curr_pos += NORTH_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + WEST_GC)) {
                    attacker_turns.direction.push(Direct::WEST);
                    curr_dir = Direct::WEST;
                    turn_locations_gc.push(curr_pos);
                    curr_pos += WEST_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + EAST_GC)) {
                    attacker_turns.direction.push(Direct::EAST);
                    curr_dir = Direct::EAST;
                    turn_locations_gc.push(curr_pos);
                    curr_pos += EAST_GC;
                } else { done = true; }
            },
            Direct::SOUTH => {
                // info!("matched SOUTH");
                if attack_tiles_hs.contains(&(curr_pos + SOUTH_GC)) {
                    // info!("move SOUTH");
                    curr_pos += SOUTH_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + WEST_GC)) {
                    // info!("turn WEST");
                    attacker_turns.direction.push(Direct::WEST);
                    curr_dir = Direct::WEST;
                    turn_locations_gc.push(curr_pos);
                    curr_pos += WEST_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + EAST_GC)) {
                    // info!("turn EAST");
                    attacker_turns.direction.push(Direct::EAST);
                    curr_dir = Direct::EAST;
                    turn_locations_gc.push(curr_pos);
                    curr_pos += EAST_GC;
                } else { done = true; }
            },
            Direct::EAST => {
                // info!("matched EAST");
                if attack_tiles_hs.contains(&(curr_pos + EAST_GC)) {
                    // info!("move EAST");
                    curr_pos += EAST_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + NORTH_GC)) {
                    // info!("turn NORTH");
                    attacker_turns.direction.push(Direct::NORTH);
                    curr_dir = Direct::NORTH;
                    turn_locations_gc.push(curr_pos);
                    curr_pos += NORTH_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + SOUTH_GC)) {
                    // info!("turn SOUTH");
                    attacker_turns.direction.push(Direct::SOUTH);
                    curr_dir = Direct::SOUTH;
                    turn_locations_gc.push(curr_pos);
                    curr_pos += SOUTH_GC;
                } else { done = true; }
            },
            Direct::WEST => {
                // info!("matched WEST");
                if attack_tiles_hs.contains(&(curr_pos + WEST_GC)) {
                    curr_pos += WEST_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + SOUTH_GC)) {
                    // info!("turn SOUTH");
                    attacker_turns.direction.push(Direct::SOUTH);
                    curr_dir = Direct::SOUTH;
                    turn_locations_gc.push(curr_pos);
                    curr_pos += SOUTH_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + NORTH_GC)) {
                    // info!("turn NORTH");
                    attacker_turns.direction.push(Direct::NORTH);
                    curr_dir = Direct::NORTH;
                    turn_locations_gc.push(curr_pos);
                    curr_pos += NORTH_GC;
                } else { done = true; }
            },
        }
    }

    // properly format x,y positions
    for gc in turn_locations_gc.iter() {
        attacker_turns.turn_locations.push(gridcoords_to_xy(gc));
    }

    turns_res.direction = attacker_turns.direction.clone();
    turns_res.turn_locations = attacker_turns.turn_locations.clone();
    // info!("attacker turn locations: {:?}", attacker_turns.turn_locations);
    // info!("attacker turn directions: {:?}", attacker_turns.direction);
}

fn gridcoords_to_xy(gc: &GridCoords) -> (i32, i32) {
    ((gc.x*64+32) as i32, (gc.y*64+32) as i32)
}

pub trait Turning {
    fn turn_left(&self) -> IVec2;
    fn turn_right(&self) -> IVec2;
}

// Planning to use this trait for quickly updating the new direction based on turns :)
impl Turning for GridCoords {
    fn turn_left(&self) -> IVec2 {
        if self.x == 1 {
            IVec2::new(0, -1)
        }
        else if self.x == -1 {
            IVec2::new(0, 1)
        }
        else if self.y == 1 {
            IVec2::new(1, 0)
        }
        else {
            IVec2::new(-1, 0)
        }
    }

    fn turn_right(&self) -> IVec2 {
        if self.x == 1 {
            IVec2::new(0, 1)
        }
        else if self.x == -1 {
            IVec2::new(0, -1)
        }
        else if self.y == 1 {
            IVec2::new(-1, 0)
        }
        else {
            IVec2::new(1, 0)
        }
    }
}