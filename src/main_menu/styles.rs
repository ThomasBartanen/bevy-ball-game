use bevy::prelude::*;

pub const MENU_BG_COLOR: Color = Color::srgba(0.4, 0.2, 0.8, 0.2);
pub const NORMAL_BUTTON_COLOR: Color = Color::srgba(0.15, 0.15, 0.15, 0.5);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgba(0.15, 0.25, 0.15, 0.75);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgba(0.15, 0.35, 0.15, 0.75);

pub const TITLE_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_content = AlignContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(500.0);
    style.height = Val::Px(120.0);
    style
};

pub const MAIN_MENU_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const BUTTON_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_content = AlignContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style
};

pub const IMAGE_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.width = Val::Px(64.0);
    style.height = Val::Px(64.0);
    style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    style
};

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
