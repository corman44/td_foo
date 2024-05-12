use bevy::prelude::*;

pub const START_BUTTON_COLOR: Color = Color::rgb_linear(0.,204.,0.);
pub const HOVER_START_BUTTON_COLOR: Color = Color::rgb_linear(0.,255.,0.);

pub const START_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.align_items = AlignItems::Center;
    style.border = UiRect::all(Val::Px(1.));
    style.height = Val::Px(25.0);
    style.justify_content = JustifyContent::Center;
    style.width = Val::Px(100.0);
    style.position_type = PositionType::Absolute;
    style.right = Val::Px(950.);
    style.top = Val::Px(512.);
    style
};

pub fn get_start_button_text_style(
    asset_server: &Res<AssetServer>,
) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Kenney Future.ttf"),
        font_size: 16.0,
        color: Color::WHITE,
    }
}
