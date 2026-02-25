// Imports
use iced::widget::{column, row, button, text, scrollable};
use iced::{Element, Length};

use crate::{Message, FileEntry};

// Define the file list UI with icons
pub fn file_list<'a>(entries: &'a [FileEntry],) -> Element<'a, Message> {
    let mut content = column![]
        .spacing(5)
        .padding(10);

    for entry in entries {
        let icon = if entry.is_dir { "FOLDER" } else { "FILE" };

        let row = row![
            text(icon),
            button(text(&entry.name.as_str()))
                .on_press(Message::EntryClicked(entry.path.clone()))
                .width(Length::Fill)
        ]
        .spacing(10);

        content = content.push(row);
    }

    scrollable(content).into()
}