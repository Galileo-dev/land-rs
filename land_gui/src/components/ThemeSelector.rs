use iced::{
    button, scrollable, slider, text_input, Align, Button, Checkbox, Column, Container, Element,
    Length, ProgressBar, Radio, Row, Rule, Sandbox, Scrollable, Settings, Slider, Space, Text,
    TextInput,
};

use crate::{
    style::{self, Theme},
    Message,
};

pub fn theme_selector<'a>(styling_theme: Theme) -> Column<'a, Message> {
    style::Theme::ALL.iter().fold(
        Column::new().spacing(2).push(Text::new("Choose a theme:")),
        |column, theme| {
            column.push(
                Radio::new(
                    *theme,
                    &format!("{:?}", theme),
                    Some(styling_theme),
                    Message::ThemeChanged,
                )
                .style(styling_theme),
            )
        },
    )
}
