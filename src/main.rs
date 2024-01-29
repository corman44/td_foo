use bevy::prelude::*;
use td_foo::AppPlugin;

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run()
}

