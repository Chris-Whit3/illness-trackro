use std::default;
use iced::alignment::Horizontal;
use iced::widget::{Button, button, column, row, container, text, text_input, Column, Container, Row, Space, theme};
use iced::{Element, Fill, Length, Theme, Alignment, Color, Background, Border, Shadow};

pub struct ITrack {
    page: Page,
    input_value: String,
    user: User,
}

pub struct User {
    name: String,
    age: i64,
    illnesses: Vec<String>,
    is_logged_in: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Page {
    #[default]Welcome,
    Search,
    IllPage(String),
    UserPage,
    // IllInfo,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToSearch,
    InputChanged(String),
    IllPageChange(String),
    GoToLogin,
    LogOut,
    GoToUserPage,
    AddIllness(String),
}

impl ITrack {

    // fn title(&self) -> String {
    //     match self.page {
    //         Page::Welcome => "Main Page".to_string(),
    //         Page::Search => "Search".to_string(),
    //         // _ => "Bad page go away".to_string(),
    //     }
    // }

    pub fn view(&self) -> Element<Message> {
        match self.page {
            Page::Welcome => self.welcome(),
            Page::Search => self.search(),
            Page::IllPage(ref illness_name) => self.illness_page(illness_name),
            Page::UserPage => {
                if self.user.is_logged_in {
                    self.user_page()
                } else {
                    self.welcome()
                }
            }
            _ => column!["bad page", "go away"].into()
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::GoToSearch => {
                self.page = Page::Search;
            }
            Message::InputChanged(input_value) => {
                self.input_value = input_value;
            }
            Message::IllPageChange(input_value) => {
                self.input_value.clear();
                self.page = Page::IllPage(input_value);
            }
            Message::GoToLogin => {
                self.user.is_logged_in = true;
            }
            Message::LogOut => {
                self.user.is_logged_in = false;
            }
            Message::GoToUserPage => {
                self.page = Page::UserPage
            }
            Message::AddIllness(illness_name) => {
                self.user.illnesses.push(illness_name.to_string())
            }
            _ => println!("error"),
        }
    }

    fn welcome(&self) -> Element<Message> {

        let account_options: Row<Message>;

        let account_button: Button<Message>;

        if self.user.is_logged_in {
            account_options = row![button(text("Log out"))
                                    .on_press(Message::LogOut)
                                    .style(button::text)];
            account_button = button(text("Account"))
                                .on_press(Message::GoToUserPage)
        } else {
            account_options = row![
                button(text("Login")).on_press(Message::GoToLogin),
                button(text("Sign Up")).on_press(Message::GoToLogin),
            ];

            account_button = button(text("Account"))
        }

        Container::new(
            row![
                Space::with_width(Length::FillPortion(1)),
                column![
                    text("Welcome"),
                    account_options,
                    button("goto search page")
                        .on_press(Message::GoToSearch)]
                    .spacing(10)
                    .align_x(Alignment::Center)
                    .width(Length::FillPortion(3)),
                account_button
                    .width(Length::FillPortion(1))])
        .into()
    }

    fn search(&self) -> Element<Message> {
        let value = &self.input_value;

        let account_button: Button<Message>;

        let mut boxes:Column<Message> = column![];

        if self.user.is_logged_in {
            account_button = button(text("Account"))
                                .on_press(Message::GoToUserPage);
        } else {
            account_button = button(text("Account"));
        }

        for ch in value.chars() {
            boxes = boxes.push(
                button(text(ch.to_string()))
                    .on_press(Message::IllPageChange(ch.to_string()))
                    .style(link_type_button));
        }

        Container::new(
            column![
                row![
                    Space::with_width(Length::FillPortion(1)),
                    text_input("Search For Your Illness", value)
                        .on_input(Message::InputChanged)
                        .padding(10)
                        .size(20)
                        .width(Length::FillPortion(3)),
                    account_button.width(Length::FillPortion(1))]
                ]
                .push(row![
                    Space::with_width(Length::FillPortion(1)),
                    Container::new(boxes)
                        .style(container::bordered_box)
                        .width(Length::FillPortion(3)),
                        
                    Space::with_width(Length::FillPortion(1))]
            ))
            .into()
    }

    fn user_page(&self) -> Element<Message> {

        let mut illness_column: Column<Message> = column![];

        for illness_name in self.user.illnesses.clone() {
            illness_column = illness_column.push(
            button(text(illness_name
                            .to_string()))
                        .on_press(Message::IllPageChange(illness_name.to_string())))
        }

        Container::new(
            row![
                column![
                    text(format!("User: {}", self.user.name)),
                    text(format!("Age: {}", self.user.age))
                ]
                    .width(Length::FillPortion(1)),
                column![
                    text("Illnesses:"),
                    illness_column,
                ]
                    .width(Length::FillPortion(3))
                    .align_x(Alignment::Center),
                button(text("Return to Search")).on_press(Message::GoToSearch).
                    width(Length::FillPortion(1)),
            ]
        )
            .center_x(Fill)
            .into()
    }

    fn illness_page(&self, ref illness_name: &String) -> Element<Message> {
        
        let add_illness_button: Button<Message>;

        if self.user.illnesses.contains(illness_name) {
            add_illness_button = button(text(illness_name.to_string()))
                                    .style(link_type_button);
        } else {
            add_illness_button = button(text(illness_name.to_string()))
                                    .on_press(Message::AddIllness(illness_name.to_string()))
                                    .style(link_type_button);
        }

        Container::new(
            column![add_illness_button,
                    button("back to search")
                        .on_press(Message::GoToSearch)])
            .into()
    }

    
}

pub fn link_type_button(theme: &Theme, status: button::Status) -> button::Style {

    match status {

        button::Status::Active | button::Status::Pressed => {
            button::Style {
                background: None,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.into(),
                },
                text_color: Color::BLACK,
                shadow: Shadow {
                    color: Color::TRANSPARENT,
                    offset: iced::Vector { x: 0.0, y: 0.0 },
                    blur_radius: 0.0,
                }
            }
        }
        button::Status::Hovered => {
            button::Style {
                background: None,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.into(),
                },
                text_color: Color {r: 0.5, g: 0.5, b: 0.5, a: 100.0},
                shadow: Shadow {
                    color: Color::TRANSPARENT,
                    offset: iced::Vector { x: 0.0, y: 0.0 },
                    blur_radius: 5.0,
                }
            }
        }
        button::Status::Disabled => {
            button::Style {
                background: None,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.into(),
                },
                text_color: Color {r: 0.75, g: 0.75, b: 0.75, a: 100.0},
                shadow: Shadow {
                    color: Color::TRANSPARENT,
                    offset: iced::Vector { x: 0.0, y: 0.0 },
                    blur_radius: 0.0,
                }
            }
        }

    }

}


fn main() -> iced::Result {
    iced::application("test header", ITrack::update, ITrack::view).run()
}

impl Default for ITrack {
    fn default() -> Self {
        Self {
            page: Page::Welcome,
            input_value: String::new(),
            user: User::default(),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: String::from("tmp_name"),
            age: -1,
            illnesses: vec![],
            is_logged_in: false,
        }
    }
}
