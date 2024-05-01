use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use bevy_ecs_ldtk::GridCoords;

use crate::game::map::AttackerArea;
use crate::game::attacker::Direct;

use super::Tank;

const NORTH: GridCoords = GridCoords{ x:0, y:1 };
const EAST: GridCoords = GridCoords{ x:1, y:0 };
const SOUTH: GridCoords = GridCoords{ x:0, y:-1 };
const WEST: GridCoords = GridCoords{ x:-1, y:0 };

pub fn move_attackers(
    mut attacker_query: Query<(&mut GridCoords, &mut Direct), (With<Tank>, Without<AttackerArea>)>,
    attack_tiles: Query<&GridCoords, (With<AttackerArea>, Without<Tank>)>,
) {
    let tiles: HashSet<GridCoords> = attack_tiles.iter().copied().collect();
    for (mut tank_grid_coords, mut tank_direct) in attacker_query.iter_mut() {
        if tank_direct.grid_coords == SOUTH {
            if tiles.contains(&(tank_direct.grid_coords + *tank_grid_coords)) {
                *tank_grid_coords += tank_direct.grid_coords;
            }
            else if tiles.contains(&(*tank_grid_coords + EAST)) {
                *tank_grid_coords += EAST;
                tank_direct.grid_coords = EAST;
            }
            else if tiles.contains(&(*tank_grid_coords + WEST)) {
                *tank_grid_coords += WEST;
                tank_direct.grid_coords = WEST;
            }
        }
        else if tank_direct.grid_coords == EAST {
            if tiles.contains(&(tank_direct.grid_coords + *tank_grid_coords)) {
                *tank_grid_coords += tank_direct.grid_coords;
            }
            else if tiles.contains(&(*tank_grid_coords + NORTH)) {
                *tank_grid_coords += NORTH;
                tank_direct.grid_coords = NORTH;
            }
            else if tiles.contains(&(*tank_grid_coords + SOUTH)) {
                *tank_grid_coords += SOUTH;
                tank_direct.grid_coords = SOUTH;
            }
        }
        else if tank_direct.grid_coords == WEST {
            if tiles.contains(&(tank_direct.grid_coords + *tank_grid_coords)) {
                *tank_grid_coords += tank_direct.grid_coords;
            }
            else if tiles.contains(&(*tank_grid_coords + NORTH)) {
                *tank_grid_coords += NORTH;
                tank_direct.grid_coords = NORTH;
            }
            else if tiles.contains(&(*tank_grid_coords + SOUTH)) {
                *tank_grid_coords += SOUTH;
                tank_direct.grid_coords = SOUTH;
            }
        }
        else if tank_direct.grid_coords == NORTH {
            if tiles.contains(&(tank_direct.grid_coords + *tank_grid_coords)) {
                *tank_grid_coords += tank_direct.grid_coords;
            }
            else if tiles.contains(&(*tank_grid_coords + EAST)) {
                *tank_grid_coords += EAST;
                tank_direct.grid_coords = EAST;
            }
            else if tiles.contains(&(*tank_grid_coords + WEST)) {
                *tank_grid_coords += WEST;
                tank_direct.grid_coords = WEST;
            }
        }
    }
}