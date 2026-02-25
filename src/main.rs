// Imports
mod filesystem;
mod ui;

use filesystem::{read_directory, FileEntry};
use std::time::{Duration, Instant};

use iced::{
    Application, Command, Element, Length, Size,
};
use iced::widget::{column, row};

use std::path::PathBuf;

// main() starts the application
fn main() -> iced::Result {
    FileExplorer::run(iced::Settings{
        window: iced::window::Settings{
            size: Size::new(900.0, 700.0),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
}

// Structs
#[derive(Debug, Clone)]
pub enum Message {
    LoadDirectory(PathBuf),
    DirectoryLoaded(Result<Vec<FileEntry>, String>),
    EntryClicked(PathBuf),
    OpenFile(PathBuf),
    GoBack,
    GoForward,
    GoUp,
    PathChanged(String),
    PathSubmitted,
}

struct FileExplorer {
    current_path: PathBuf,
    entries: Vec<FileEntry>,
    loading: bool,
    path_input: String,
    // Handle for double click
    selected: Option<PathBuf>,
    last_click_time: Option<Instant>,
}

impl Application for FileExplorer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let path = std::env::current_dir().unwrap_or_default();

        (
            Self {
                current_path: path.clone(),
                entries: vec![],
                loading: true,
                path_input: path.to_string_lossy().to_string(),
                selected: None,
                last_click_time: None,
            },
            Command::perform(read_directory(path), Message::DirectoryLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Rust File Explorer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadDirectory(path) => {
                self.loading = true;
                self.current_path = path.clone();
                self.path_input = path.to_string_lossy().to_string();

                Command::perform(read_directory(path), Message::DirectoryLoaded)
            }

            Message::DirectoryLoaded(result) => {
                self.loading = false;
                self.selected = None;
                if let Ok(entries) = result {
                    self.entries = entries;
                }
                Command::none()
            }

            Message::EntryClicked(path) => {
                let now = Instant::now();
                let double_click_threshold = Duration::from_millis(500);

                if let Some(selected_path) = &self.selected {
                    if *selected_path == path {
                        if let Some(last_time) = self.last_click_time {
                            if now.duration_since(last_time) < double_click_threshold {
                                // DOUBLE CLICK DETECTED
                                if path.is_dir() {
                                    self.loading = true;
                                    self.current_path = path.clone();
                                    self.path_input = path.to_string_lossy().to_string();

                                    return Command::perform(
                                        read_directory(path),
                                        Message::DirectoryLoaded,
                                    );
                                } else {
                                    return Command::perform(
                                        async move {
                                            opener::open(path).map_err(|e| e.to_string())
                                        },
                                        |_| Message::OpenFile(PathBuf::new()),
                                    );
                                }
                            }
                        }
                    }
                }

                // Otherwise: single click → just select
                self.selected = Some(path);
                self.last_click_time = Some(now);

                Command::none()
            }
            
            Message::OpenFile(_) => {
                // Nothing to update in UI, file openning handled by the OS
                Command::none()
            }

            Message::GoUp => {
                if let Some(parent) = self.current_path.parent() {
                    let path = parent.to_path_buf();
                    self.current_path = path.clone();                 // update current_path
                    self.path_input = path.to_string_lossy().to_string(); // update input field

                    return Command::perform(
                        read_directory(path),
                        Message::DirectoryLoaded,
                    );
                }
                Command::none()
            }

            Message::PathChanged(new_path) => {
                self.path_input = new_path;
                Command::none()
            }

            Message::PathSubmitted => {
                let path = PathBuf::from(self.path_input.clone());
                Command::perform(read_directory(path), Message::DirectoryLoaded)
            }

            _ => Command::none(),
        }
    }

    // Load the view
    fn view<'a>(&'a self) -> Element<'a, Message> {
        column![
            ui::toolbar::toolbar(&self.path_input),
            row![
                ui::sidebar::sidebar(),
                ui::file_list::file_list(&self.entries),
            ]
            .height(Length::Fill)
        ]
        .into()
    }
}
