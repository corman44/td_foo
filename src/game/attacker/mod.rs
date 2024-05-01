
pub mod systems;
pub mod components;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use self::systems::move_attackers;
use super::GameState;

pub struct AttackerPlugin;

impl Plugin for AttackerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<RedTankBundle>("RedTank")
            .register_ldtk_entity::<BlueTankBundle>("BlueTank")
            .add_systems(OnEnter(GameState::Running), move_attackers);
    }
}

#[derive(Default, Component)]
pub struct Tank;

#[derive(Default, Bundle, LdtkEntity)]
struct RedTankBundle {
    tank: Tank,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    direction: Direct,
}

#[derive(Default, Bundle, LdtkEntity)]
struct BlueTankBundle {
    tank: Tank,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    direction: Direct,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
pub struct Direct {
    grid_coords: GridCoords,
}

impl Default for Direct {
    fn default() -> Self {
        Self { grid_coords: GridCoords::new(0,-1) }
    }
}