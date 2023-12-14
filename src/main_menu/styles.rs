use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const CLICKED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(50.0);
    style.height = Val::Percent(20.0);
    style.column_gap = Val::Px(20.0);
    style
};

pub const IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.width = Val::Percent(15.0);
    style.height = Val::Percent(45.0);
    // style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    style
};

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(30.0);
    style.height = Val::Percent(10.0);
    style
};

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
    }
}
