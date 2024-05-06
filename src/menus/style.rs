use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const MARGIN: Val = Val::Px(2.);

pub const NORMAL_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.align_items = AlignItems::Center;
    style.border = UiRect::all(Val::Px(2.));
    style.column_gap = Val::Px(10.0);
    style.height = Val::Px(100.0);
    style.justify_content = JustifyContent::Center;
    style.width = Val::Px(350.0);
    style
};

pub const IMAGE_STYLE:  Style = {
    let mut style = Style::DEFAULT;
    style.height = Val::Px(64.0);
    style.width =  Val::Px(64.0);
    style.margin = UiRect::new(
        Val::Px(8.0),
        Val::Px(8.0), 
        Val::Px(8.0),
        Val::Px(8.0),
    );
    style
};

pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.border = UiRect::all(MARGIN);
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(50.0);
    style.column_gap = Val::Px(50.0);
    style
};

pub const TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.border = UiRect::all(MARGIN);
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.justify_items = JustifyItems::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(500.0);
    style.height = Val::Px(120.0);
    style
};

pub const PAUSE_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.border = UiRect::all(MARGIN);
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(50.0);
    style.column_gap = Val::Px(50.0);
    style
};

pub fn get_title_text_style(
    asset_server: &Res<AssetServer>,
) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Kenney Future.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
    }
}

pub fn get_pause_text_style(
    asset_server: &Res<AssetServer>,
) -> TextStyle {
    TextStyle { 
        font: asset_server.load("fonts/Kenney Future.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
     }
}

pub fn get_button_text_style(
    asset_server: &Res<AssetServer>,
) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Kenney Future.ttf"),
        font_size: 48.0,
        color: Color::WHITE,
    }
}
