use bevy::prelude::*;

pub fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        eprintln!("MouseButton::Left Pressed");
    }
    if buttons.just_released(MouseButton::Left) {
        eprintln!("MouseButton::Left Release");
    }
    if buttons.pressed(MouseButton::Right) {
        eprintln!("MouseButton::Right Pressed");
    }
    // we can check multiple at once with `.any_*`
    // if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
    //     // Either the left or the right button was just pressed
    // }
}