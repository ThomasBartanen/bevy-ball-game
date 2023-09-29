use bevy::prelude::*;

pub const DATA_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceEvenly;
    style.align_content = AlignContent::Start;
    style.align_items = AlignItems::Start;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub const HUD_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceEvenly;
    style.align_items = AlignItems::Stretch;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};
/*
pub const IMAGE_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.width = Val::Px(64.0);
    style.height = Val::Px(64.0);
    style.margin = UiRect::new(
        Val::Px(8.0), 
        Val::Px(8.0), 
        Val::Px(8.0), 
        Val::Px(8.0));
    style
};
*/
pub fn get_data_text_style(
    asset_server: &Res<AssetServer>
) -> TextStyle {
    TextStyle { 
        font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
        font_size: 28.0, 
        color: Color::WHITE 
    }
}