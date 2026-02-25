// Imports
use iced::widget::{row, button, text_input};
use iced::{Element, Length};

use crate::Message;

// Define the toolbar UI
pub fn toolbar<'a>(current_path: &str,) -> Element<'a, Message> {
    row![
        button("UP").on_press(Message::GoUp),
        text_input("Path...", current_path)
            .on_input(Message::PathChanged)
            .on_submit(Message::PathSubmitted),
    ]
    .spacing(10)
    .padding(10)
    .height(Length::Shrink)
    .into()
}