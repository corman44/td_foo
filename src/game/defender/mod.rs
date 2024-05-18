pub mod systems;
pub mod components;

use std::ops::SubAssign;

use bevy::prelude::*;

pub struct DefenderPlugin;

impl Plugin for DefenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DefenderStats::default());
    }
}


#[derive(Component, Debug)]
pub struct Coins(pub i32);

impl Default for Coins {
    fn default() -> Self {
        Self(20)
    }
}

#[derive(Component, Debug)]
pub struct Health(pub i32);

impl Default for Health {
    fn default() -> Self {
        Self(100)
    }
}

impl SubAssign for Health {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

#[derive(Resource, Debug, Default)]
pub struct DefenderStats {
    pub coins: Coins,
    pub health: Health,
}
