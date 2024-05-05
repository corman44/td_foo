use bevy::prelude::*;
use td_foo::AppPlugin;

// TODO: add mouse events/mouse tracker

/* --- Developer Info ---

    * Controls
        M -> Main Menu
        G -> Game
        P -> toggle Game Paused and Running
        S -> Spawn Tank at starting pos
        N -> Move Tank forward
 */

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run()
}