use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Tile {
    Path,
    Tower,
    AttackStart,
    AttackEnd,
    Empty
}

#[derive(Debug, Resource)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub height: usize,
    pub width: usize,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let map = Map {
            tiles: vec![],
            height: 20,
            width: 20,
        };
        app.insert_resource(map);
    }
}

impl Map {
    // TODO: implement map creation/updates
}