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

pub const CONTROLS_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::SpaceEvenly;
    style.align_content = AlignContent::SpaceEvenly;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.align_self = AlignSelf::FlexStart;
    style
};

pub const COMPLETE_HUD_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub const TOP_HUD_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceEvenly;
    style.align_items = AlignItems::Stretch;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub const SIDE_HUD_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.align_self = AlignSelf::End;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Start;
    style.align_items = AlignItems::Start;
    style.width = Val::Px(100.0);
    style.height = Val::Percent(60.0);
    //style.border = UiRect::new(Val::Px(3.0), Val::Px(3.0), Val::Px(3.0), Val::Px(3.0));
    //style.flex_wrap = FlexWrap::Wrap;
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

pub fn get_controls_text_style(
    asset_server: &Res<AssetServer>
) -> TextStyle {
    TextStyle { 
        font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
        font_size: 25.0, 
        color: Color::WHITE        
    }
}