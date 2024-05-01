pub mod system;
pub mod components;

use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_ldtk::prelude::*;

use crate::AppState;

use self::system::{debug_cameras, map_setup, translate_grid_coords_entities, zoom_to_map};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LdtkPlugin)
            .add_systems(Startup, map_setup)
            .insert_resource(LevelSelection::index(0))
            .add_systems(OnEnter(AppState::Game), (debug_cameras, zoom_to_map))
            .add_systems(Update, translate_grid_coords_entities)
            .register_ldtk_int_cell::<DefenderAreaBundle>(1)
            .register_ldtk_int_cell::<AttackerAreaBundle>(2)
            .register_ldtk_int_cell::<UnusedAreaBundle>(3);
        }
}


#[derive(Default, Component)]
pub struct DefenderArea;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct DefenderAreaBundle {
    DefenderArea: DefenderArea,
}

#[derive(Default, Component)]
pub struct AttackerArea;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct AttackerAreaBundle {
    AttackerArea: AttackerArea,
}

#[derive(Default, Component)]
pub struct UnusedArea;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct UnusedAreaBundle {
    UnusedArea: UnusedArea,
}

#[derive(Default, Resource)]
pub struct AttackTiles {
    locations: HashSet<GridCoords>,
}

impl AttackTiles {
    fn is_attack_tile(&self, grid_coords: &GridCoords) -> bool {
        self.locations.contains(grid_coords)
    }
}

#[derive(Default, Resource)]
pub struct DefendTiles {
    locations: HashSet<GridCoords>,
}

#[derive(Default, Resource)]
pub struct UnusedTiles {
    locations: HashSet<GridCoords>,
}

#[derive(Default, Component)]
struct Tile;

#[derive(Default, Bundle, LdtkIntCell)]
struct TileBundle {
    tile: Tile,
}
