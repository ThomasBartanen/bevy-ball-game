use bevy::prelude::*;

pub const DATA_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceEvenly;
    style.align_content = AlignContent::Start;
    style.align_items = AlignItems::Start;
    style.column_gap = Val::Px(100.0);
    style.width = Val::Px(500.0);
    style.height = Val::Px(120.0);
    style
};

pub const HUD_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceEvenly;
    style.align_items = AlignItems::Stretch;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(20.0);
    style.column_gap = Val::Px(20.0);
    style
};

pub fn get_data_text_style(
    asset_server: &Res<AssetServer>
) -> TextStyle {
    TextStyle { 
        font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
        font_size: 28.0, 
        color: Color::WHITE 
    }
}