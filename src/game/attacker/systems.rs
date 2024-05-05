use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use bevy_ecs_ldtk::prelude::*;

use crate::game::map::AttackerArea;
use crate::game::attacker::Direct;

use super::{RedTankAttacker, Tank};

const NORTH: GridCoords = GridCoords{ x:0, y:1 };
const EAST: GridCoords = GridCoords{ x:1, y:0 };
const SOUTH: GridCoords = GridCoords{ x:0, y:-1 };
const WEST: GridCoords = GridCoords{ x:-1, y:0 };

pub fn move_attackers(
    mut attacker_query: Query<(&mut GridCoords, &mut Direct, &mut Transform), (With<Tank>, Without<AttackerArea>)>,
    attack_tiles: Query<&GridCoords, (With<AttackerArea>, Without<Tank>)>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::N) {
        let tiles: HashSet<GridCoords> = attack_tiles.iter().copied().collect();
        for (mut tank_grid_coords, mut tank_direct, mut tank_transform) in attacker_query.iter_mut() {
            if tank_direct.grid_coords == SOUTH {
                if tiles.contains(&(tank_direct.grid_coords + *tank_grid_coords)) {
                    *tank_grid_coords += tank_direct.grid_coords;
                }
                else if tiles.contains(&(*tank_grid_coords + EAST)) {
                    *tank_grid_coords += EAST;
                    tank_direct.grid_coords = EAST;
                    tank_transform.rotate_z(-PI/2.);
                }
                else if tiles.contains(&(*tank_grid_coords + WEST)) {
                    *tank_grid_coords += WEST;
                    tank_direct.grid_coords = WEST;
                    tank_transform.rotate_z(PI/2.);
                }
            }
            else if tank_direct.grid_coords == EAST {
                if tiles.contains(&(tank_direct.grid_coords + *tank_grid_coords)) {
                    *tank_grid_coords += tank_direct.grid_coords;
                }
                else if tiles.contains(&(*tank_grid_coords + NORTH)) {
                    *tank_grid_coords += NORTH;
                    tank_direct.grid_coords = NORTH;
                    tank_transform.rotate_z(PI/2.);
                }
                else if tiles.contains(&(*tank_grid_coords + SOUTH)) {
                    *tank_grid_coords += SOUTH;
                    tank_direct.grid_coords = SOUTH;
                    tank_transform.rotate_z(-PI/2.);
                }
            }
            else if tank_direct.grid_coords == WEST {
                if tiles.contains(&(tank_direct.grid_coords + *tank_grid_coords)) {
                    *tank_grid_coords += tank_direct.grid_coords;
                }
                else if tiles.contains(&(*tank_grid_coords + NORTH)) {
                    *tank_grid_coords += NORTH;
                    tank_direct.grid_coords = NORTH;
                    tank_transform.rotate_z(-PI/2.);
                }
                else if tiles.contains(&(*tank_grid_coords + SOUTH)) {
                    *tank_grid_coords += SOUTH;
                    tank_direct.grid_coords = SOUTH;
                    tank_transform.rotate_z(PI/2.);
                }
            }
            else if tank_direct.grid_coords == NORTH {
                if tiles.contains(&(tank_direct.grid_coords + *tank_grid_coords)) {
                    *tank_grid_coords += tank_direct.grid_coords;
                }
                else if tiles.contains(&(*tank_grid_coords + EAST)) {
                    *tank_grid_coords += EAST;
                    tank_direct.grid_coords = EAST;
                    tank_transform.rotate_z(-PI/2.);
                }
                else if tiles.contains(&(*tank_grid_coords + WEST)) {
                    *tank_grid_coords += WEST;
                    tank_direct.grid_coords = WEST;
                    tank_transform.rotate_z(PI/2.);
                }
            }
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
                transform: Transform { translation: Vec3::new(0., 0., 6.), ..default() },
                ..default()
            },
            grid_coords: GridCoords { x:7, y:15 }, // starting position for basic map
            direction: Direct { grid_coords: SOUTH },
            ..default()
        }).id();
        eprintln!("RedTankAttacker spawned: id={}",id.index());
    }
}
