use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::{attacker::TankMovement, map::AttackerArea};

use super::{AttackerTurns, Direct, Direction, RedTankAttacker, Tank};

const NORTH_IV: IVec2 = IVec2{ x:0, y:1 };
const EAST_IV: IVec2 = IVec2{ x:1, y:0 };
const SOUTH_IV: IVec2 = IVec2{ x:0, y:-1 };
const WEST_IV: IVec2 = IVec2{ x:-1, y:0 };
const NORTH_GC: GridCoords = GridCoords{ x:0, y:-1 };
const EAST_GC: GridCoords = GridCoords{ x:1, y:0 };
const SOUTH_GC: GridCoords = GridCoords{ x:0, y:1 };
const WEST_GC: GridCoords = GridCoords{ x:-1, y:0 };
const SPEED: f32 = 100.;

pub fn move_attackers(
    mut attacker_query: Query<(&mut TankMovement, &mut Transform)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    // TODO: check if new transform is past turning point, if so apply direciton until turn, then apply remaining in new direction
    for (mut tank_movement, mut tank_transform) in attacker_query.iter_mut() {
        match tank_movement.direction {
            Direct::NORTH => {
                tank_transform.translation.y += delta * SPEED ;
            },
            Direct::SOUTH => {
                tank_transform.translation.y -= delta * SPEED ;
            },
            Direct::EAST => {
                tank_transform.translation.x += delta * SPEED ;
            },
            Direct::WEST => {
                tank_transform.translation.x -= delta * SPEED ;
            },
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
        info!("RedTankAttacker spawned: id={}",id.index());
    }
}

pub fn init_attacker_turns(
    mut turns_res: ResMut<AttackerTurns>,
    attack_tiles_query: Query<&GridCoords, With<AttackerArea>>,
) {
    let attack_tiles_hs: HashSet<GridCoords> = attack_tiles_query.iter().copied().collect();
    let mut attacker_turns = AttackerTurns { turn_locations: vec![], ..default()};
    let mut turn_locations_gc: Vec<GridCoords> = vec![];
    let mut curr_pos = GridCoords {x: 7, y: 0};
    let mut curr_dir = Direct::SOUTH;
    let mut done: bool = false;

    while(!done) {
        // if dir south and found south, update curr_pos and continue.
        // otherwise, turn found and need to push the turn info to attacker_turns
        match curr_dir {
            Direct::NORTH => {
                if attack_tiles_hs.contains(&(curr_pos + NORTH_GC)) {
                    curr_pos += NORTH_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + WEST_GC)) {
                    attacker_turns.direction.push(Direct::WEST);
                    turn_locations_gc.push(curr_pos);
                    curr_pos.turn_left();
                } else if attack_tiles_hs.contains(&(curr_pos + EAST_GC)) {
                    attacker_turns.direction.push(Direct::EAST);
                    turn_locations_gc.push(curr_pos);
                    curr_pos.turn_right();
                } else { done = true; }
            },
            Direct::SOUTH => {
                if attack_tiles_hs.contains(&(curr_pos + SOUTH_GC)) {
                    curr_pos += SOUTH_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + WEST_GC)) {
                    attacker_turns.direction.push(Direct::WEST);
                    turn_locations_gc.push(curr_pos);
                    curr_pos.turn_right();
                } else if attack_tiles_hs.contains(&(curr_pos + EAST_GC)) {
                    attacker_turns.direction.push(Direct::EAST);
                    turn_locations_gc.push(curr_pos);
                    curr_pos.turn_left();
                } else { done = true; }
            },
            Direct::EAST => {
                if attack_tiles_hs.contains(&(curr_pos + EAST_GC)) {
                    curr_pos += EAST_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + NORTH_GC)) {
                    attacker_turns.direction.push(Direct::NORTH);
                    turn_locations_gc.push(curr_pos);
                    curr_pos.turn_left();
                } else if attack_tiles_hs.contains(&(curr_pos + SOUTH_GC)) {
                    attacker_turns.direction.push(Direct::SOUTH);
                    turn_locations_gc.push(curr_pos);
                    curr_pos.turn_right();
                } else { done = true; }
            },
            Direct::WEST => {
                if attack_tiles_hs.contains(&(curr_pos + WEST_GC)) {
                    curr_pos += WEST_GC;
                } else if attack_tiles_hs.contains(&(curr_pos + SOUTH_GC)) {
                    attacker_turns.direction.push(Direct::SOUTH);
                    turn_locations_gc.push(curr_pos);
                    curr_pos.turn_left();
                } else if attack_tiles_hs.contains(&(curr_pos + NORTH_GC)) {
                    attacker_turns.direction.push(Direct::NORTH);
                    turn_locations_gc.push(curr_pos);
                    curr_pos.turn_right();
                } else { done = true; }
            },
        }
    }

    let _ = turn_locations_gc.iter().map(|gc| {
        attacker_turns.turn_locations.push(gridcoords_to_xy(&gc))
    });

    turns_res.direction = attacker_turns.direction;
    turns_res.turn_locations = attacker_turns.turn_locations;
}

fn gridcoords_to_xy(gc: &GridCoords) -> (i32, i32) {
    ((gc.x*64+32) as i32, ((gc.y-15).abs()*64 +32) as i32)
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