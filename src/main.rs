use bevy::prelude::*;
use td_foo::AppPlugin;

// TODO: rotate Attackers as it changes direction
// TODO: spawn more Attackers after X time
// TODO: add mouse events/mouse tracker

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run()
}
