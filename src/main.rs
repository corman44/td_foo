use bevy::prelude::*;
use td_foo::AppPlugin;

// TODO: add mouse events/mouse tracker

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run()
}