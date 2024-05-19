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


#[derive(Component, Debug, PartialEq, PartialOrd)]
pub struct Coins(pub i32);

impl Default for Coins {
    fn default() -> Self {
        Self(20)
    }
}

#[derive(Component, Debug, PartialEq, PartialOrd)]
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

#[derive(Component, Default, Debug)]
pub struct Defender{
    attacking: bool,
    attack_speed: i32,
    damage: i32,
}

#[derive(Component)]
pub struct Active(bool);

impl Default for Active {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Bundle, Default)]
pub struct DefenderBundle {
    active: Active,
    defender: Defender,
    sprite_bundle: SpriteBundle, 
}
