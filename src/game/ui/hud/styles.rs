use bevy::prelude::*;

pub const HUD_MENU_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.), Val::Px(80.)),
    gap: Size::new(Val::Px(8.), Val::Px(8.)),
    ..Style::DEFAULT
};

pub const HUD_TEXT_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(300.), Val::Px(40.)),
    gap: Size::new(Val::Px(8.), Val::Px(8.)),
    ..Style::DEFAULT
};
