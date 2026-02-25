// Imports
use iced::widget::{column, button, text};
use iced::{Element, Length};

use crate::Message;
use std::path::PathBuf;

// Define the sidebar UI, shortcut
pub fn sidebar<'a>() -> Element<'a, Message> {
    column![
        text("Quick Access"),
        button("Home")
            .on_press(Message::LoadDirectory(home_dir())),
    ]
    .spacing(10)
    .padding(10)
    .width(Length::Fixed(200.0))
    .into()
}

fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
}