use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use bevy_ecs_ldtk::prelude::*;

use crate::game::map::AttackerArea;
use crate::game::attacker::Direct;

use super::{RedTankAttacker, Tank};

const NORTH: IVec2 = IVec2{ x:0, y:1 };
const EAST: IVec2 = IVec2{ x:1, y:0 };
const SOUTH: IVec2 = IVec2{ x:0, y:-1 };
const WEST: IVec2 = IVec2{ x:-1, y:0 };
const SPEED: f32 = 100.;

pub fn move_attackers(
    mut attacker_query: Query<(&mut Direct, &mut Transform), (With<Tank>, Without<AttackerArea>)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    for (mut tank_direct, mut tank_transform) in attacker_query.iter_mut() {
        tank_transform.translation.x += tank_direct.vec.x as f32 * delta * SPEED ;
        tank_transform.translation.y += tank_direct.vec.y as f32 * delta * SPEED ;
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
            direction: Direct { vec: IVec2 { x: 0, y: -1 }  },
            ..default()
        }).id();
        info!("RedTankAttacker spawned: id={}",id.index());
    }
}
