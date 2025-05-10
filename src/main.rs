use std::default;

use iced::widget::{button, container, column, row, text, Column};
use iced::{Fill, Element, Theme};

pub struct ITrack {
    value: i64,
    page: Page,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    #[default]Welcome,
    Search,
    // User,
    // IllInfo,
}

// impl Default for Page {
//     fn default() -> Self {
//         Page::Welcome
//     }
// }

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
    GoToSearch,
}

impl ITrack {

    // fn title(&self) -> String {
    //     match self.page {
    //         Page::Welcome => "Main Page".to_string(),
    //         Page::Search => "Search".to_string(),
    //         // _ => "Bad page go away".to_string(),
    //     }
    // }

    pub fn view(&self) -> Column<Message> {
        match self.page {
            Page::Welcome => {
                column![
                        "Top",
                        button("+").on_press(Message::Increment),
                        row![text(self.value).size(50), "Right"].spacing(90),
                        button("-").on_press(Message::Decrement),
                        "Bottom",
                    ]
                .spacing(10)
                // .padding(10)
                // .center_x(Fill)
                // .center_y(Fill)
                .push(button("goto search page").on_press(Message::GoToSearch))
            }
            Page::Search => {
                column!["tmp", "txt"]
            }
            // _ => column!["bad page", "go away"]
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::GoToSearch => {
                self.page = Page::Search;
            }
        }
    }
}

fn main() -> iced::Result {
    // iced::run("Main Page", ITrack::update, ITrack::view)
    iced::application("test header", ITrack::update, ITrack::view).run()
}

impl Default for ITrack {
    fn default() -> Self {
        Self {
            page: Page::Welcome,
            value: 0,
        }
    }
}
