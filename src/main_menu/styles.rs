use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::GRAY;
pub const NORMAL_TEXT_COLOR: Color = Color::WHITE;
pub const NORMAL_BACKGROUND_COLOR: Color = Color::DARK_GRAY;

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.), Val::Px(80.)),
    ..Style::DEFAULT
};