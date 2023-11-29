mod analog_clock;

use iced::widget::{button, column, row, text};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

// This seems like the "Model" in an MVC
struct Counter {
    hours: i32,
    minutes: i32,
    seconds: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    HourIncrementPressed,
    HourDecrementPressed,
    MinuteIncrementPressed,
    MinuteDecrementPressed,
    SecondIncrementPressed,
    SecondDecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Alarm Clock")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::HourIncrementPressed => {
                self.hours += 1;
            }
            Message::HourDecrementPressed => {
                if self.hours > 0 {
                    self.hours -= 1
                };
            }
            Message::MinuteIncrementPressed => {
                self.minutes += 1;
            }
            Message::MinuteDecrementPressed => {
                if self.minutes > 0 {
                    self.minutes -= 1
                };
            }
            Message::SecondIncrementPressed => {
                self.seconds += 1;
            }
            Message::SecondDecrementPressed => {
                if self.seconds > 0 {
                    self.seconds -= 1
                };
            }
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            column![
                button("+").on_press(Message::HourIncrementPressed),
                text(self.hours).size(50),
                button("-").on_press(Message::HourDecrementPressed),
                text("Hours").size(14)
            ]
            .padding(20)
            .align_items(Alignment::Center),
            column![
                button("+").on_press(Message::MinuteIncrementPressed),
                text(self.minutes).size(50),
                button("-").on_press(Message::MinuteDecrementPressed),
                text("Minutes").size(14)
            ]
            .padding(20)
            .align_items(Alignment::Center),
            column![
                button("+").on_press(Message::SecondIncrementPressed),
                text(self.seconds).size(50),
                button("-").on_press(Message::SecondDecrementPressed),
                text("Seconds").size(14)
            ]
            .padding(20)
            .align_items(Alignment::Center),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
