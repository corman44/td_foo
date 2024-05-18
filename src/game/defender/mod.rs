pub mod systems;
pub mod components;

use bevy::prelude::*;

pub struct DefenderPlugin;

impl Plugin for DefenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DefenderStats::default());
    }
}


#[derive(Component, Debug)]
pub struct Coins(i32);

impl Default for Coins {
    fn default() -> Self {
        Self(20)
    }
}

#[derive(Component, Debug)]
pub struct Health(i32);

impl Default for Health {
    fn default() -> Self {
        Self(100)
    }
}

#[derive(Resource, Debug, Default)]
pub struct DefenderStats {
    coins: Coins,
    health: Health,
}
