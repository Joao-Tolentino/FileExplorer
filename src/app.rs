// Imports
use iced::widget::{column, row};
use iced::{Element, Length};

use crate::ui::{toolbar, sidebar, file_list};

// Create the view in a defined Layout
fn view(&self) -> Element<Message> {
    column![
        toolbar::toolbar(self.current_path.to_str().unwrap_or("")),
        row![
            sidebar::sidebar(),
            file_list::file_list(&self.entries),
        ]
        .height(Length::Fill)
    ]
    .into()
}