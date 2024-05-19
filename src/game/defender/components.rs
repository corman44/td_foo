use std::ops::SubAssign;

use bevy::prelude::*;

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

#[derive(Component, Default, Debug)]
pub struct Defender{
    pub attacking: bool,
    pub attack_speed: i32,
    pub damage: i32,
    pub spawning: bool,
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
    pub active: Active,
    pub defender: Defender,
    pub sprite_bundle: SpriteBundle, 
}

#[derive(Resource, Debug, Default)]
pub struct DefenderStats {
    pub coins: Coins,
    pub health: Health,
}