mod analog_clock;

use std::time;

use ::time::Instant;
use iced::widget::{button, column, row, text, Row};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

// function that returns side by side +/- increment and decrement elements wrapped in a row
fn inc_dec_buttons<'a>(inc_message: Message, dec_message: Message) -> Row<'a, Message> {
    row![
        button("+").on_press(inc_message),
        button("-").on_press(dec_message),
    ]
    .padding(10)
    .spacing(1)
    .align_items(Alignment::Center)
}

// This seems like the "Model" in an MVC
struct Counter {
    hours: i32,
    minutes: i32,
    seconds: i32,
    countdown_active: bool,
    countdown_start_time: ::Instant,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    HourIncrementPressed,
    HourDecrementPressed,
    MinuteIncrementPressed,
    MinuteDecrementPressed,
    SecondIncrementPressed,
    SecondDecrementPressed,
    StartCountdown,
    StopCountdown,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            countdown_active: false,
            countdown_start_time: time::Instant::now(),
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
            Message::StartCountdown => {
                self.countdown_active = true;
                self.countdown_start_time = time::Instant::now();
            }
            Message::StopCountdown => {
                self.countdown_active = false;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                column![
                    text("Hours").size(14),
                    text(self.hours).size(50),
                    inc_dec_buttons(Message::HourIncrementPressed, Message::HourDecrementPressed),
                ]
                .padding(5)
                .align_items(Alignment::Center),
                column![
                    text("Minutes").size(14),
                    text(self.minutes).size(50),
                    inc_dec_buttons(
                        Message::MinuteIncrementPressed,
                        Message::MinuteDecrementPressed
                    ),
                ]
                .padding(5)
                .align_items(Alignment::Center),
                column![
                    text("Seconds").size(14),
                    text(self.seconds).size(50),
                    inc_dec_buttons(
                        Message::SecondIncrementPressed,
                        Message::SecondDecrementPressed
                    ),
                ]
                .padding(5)
                .align_items(Alignment::Center),
            ]
            .padding(5)
            .align_items(Alignment::Center),
            row![if self.countdown_active {
                button("Stop Alarm").on_press(Message::StopCountdown)
            } else {
                button("Start Alarm").on_press(Message::StartCountdown)
            }]
            .align_items(Alignment::Center)
        ]
        .align_items(Alignment::Center)
        .into()
    }
}
