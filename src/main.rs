use iced::{
    widget::{column, Column, text, text::Shaping, Text},
    Font, Length, Horizontal, Vertical,
};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum Message {
    _Increment,
}

fn update(_value: &mut u64, _message: Message) {}

fn view(_value: &u64) -> Column<Message> {
        column![
            "Construct from &str",
            text("Construct from function"),
            Text::new("Construct from struct"),
            text("Different font").font(Font {
                family: Family::Fantasy,
                ..Font::DEFAULT
            }),
            text("Larger text").size(24),
            text("Special character 😊").shaping(Shaping::Advanced),
            text("Center")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
            text("Vertical center")
                .height(Length::Fill)
                .vertical_alignment(Vertical::Center),
        ]
        .into()
}