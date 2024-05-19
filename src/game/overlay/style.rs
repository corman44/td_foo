use bevy::{prelude::*, ui::widget::UiImageSize};

pub const START_BUTTON_COLOR: Color = Color::GREEN;
pub const HOVER_START_BUTTON_COLOR: Color = Color::GRAY;
pub const DEFENDER1_BUTTON_COLOR: Color = Color::MIDNIGHT_BLUE;
pub const DEFENDER1_HOVER_BUTTON_COLOR: Color = Color::ALICE_BLUE;

pub const START_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.align_items = AlignItems::Center;
    style.border = UiRect::all(Val::Px(1.));
    style.height = Val::Px(25.0);
    style.justify_content = JustifyContent::Center;
    style.width = Val::Px(100.0);
    style.position_type = PositionType::Absolute;
    style.right = Val::Px(50.);
    style.top = Val::Px(512.);
    style
};

pub const DEFENDR_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.align_items = AlignItems::Center;
    style.border = UiRect::all(Val::Px(1.));
    style.height = Val::Px(25.0);
    style.justify_content = JustifyContent::Center;
    style.width = Val::Px(25.0);
    style.position_type = PositionType::Absolute;
    style.right = Val::Px(50.);
    style.top = Val::Px(600.);
    style
};

pub fn get_start_button_text_style(
    asset_server: &Res<AssetServer>,
) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Kenney Future.ttf"),
        font_size: 16.0,
        color: Color::BLACK,
    }
}

pub fn get_defender_button_image_style(
    asset_server: &Res<AssetServer>,
) -> UiImage {
    UiImage {
        texture: asset_server.load("kenney_top-down-tanks-redux/PNG/Default size/tank_blue_cj.png").into(),
        ..default()
    }
}
