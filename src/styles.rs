//! Contains common styles used in the Stargaze game.

use bevy::prelude::*;

/// The normal color of a button.
pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

/// The color of a button when it is hovered over.
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);

/// The color of a button when it is pressed.
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

/// The style for the main menu.
pub const MENU_STYLE: Style = {
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

/// The style for the title.
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

/// The style for the image.
pub const IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.width = Val::Percent(15.0);
    style.height = Val::Percent(45.0);
    style
};

/// The style for the button.
pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(30.0);
    style.height = Val::Percent(10.0);
    style.margin = UiRect {
        left: Val::Percent(0.),
        right: Val::Percent(0.),
        top: Val::Percent(0.),
        bottom: Val::Px(50.),
    };
    style
};

/// Returns the text style for a button.
pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}

/// Returns the text style for a title.
pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
    }
}

/// Returns the text style for the directions title.
pub fn get_directions_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::WHITE,
    }
}

/// Returns the text style for the directions body.
pub fn get_directions_body_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 24.0,
        color: Color::WHITE,
    }
}