use bevy::{input::{mouse::{MouseButtonInput, MouseMotion}, ButtonState}, prelude::*, window::PrimaryWindow};


pub fn find_mouse_position_on_click(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    for event in mouse_button_input_events.read() {
        info!("{:?}", event);
        if let Some(position) = q_windows.single().cursor_position() {
            info!("Mouse position: {:?}", position);
        }
    }
}

pub fn track_click_and_drag(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    _motion_evr: EventReader<MouseMotion>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let mut is_mouse_down: bool = false;

    for event in mouse_button_input_events.read() {
        if event.state == ButtonState::Pressed {
            is_mouse_down = true;
        } else {
            is_mouse_down = false;
        }
        if q_windows.single().cursor_position().is_some() && is_mouse_down {
            let position = q_windows.single().cursor_position().unwrap();
            info!("Mouse position: {:?}", position);
        }
    }
}