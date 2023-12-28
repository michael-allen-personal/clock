use std::io::BufReader;
use std::time::{Duration, Instant};

use rodio::{OutputStream, Sink};

use crate::clock;

enum TimerState {
    SetTimer(clock::ClockValue),
    Countdown(Instant, clock::ClockValue),
    PlayingTimerSound(clock::ClockValue, OutputStream, Sink),
}

impl TimerState {
    fn set_timer(clock_value: clock::ClockValue) -> Self {
        TimerState::SetTimer(clock_value)
    }

    fn start_countdown(clock_value: clock::ClockValue) -> Self {
        TimerState::Countdown(Instant::now(), clock_value)
    }

    fn play_timer_sound(clock_value: clock::ClockValue) -> Self {
        // get the default device every time, as sometimes I will change
        // output while the clock is still active. I would want the timer
        // to play out of whatever the current output device is when it
        // goes off
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        // something better needs to be done with this
        let timer_sound_file =
            std::fs::File::open("/home/michael/.clock/FinalFantasyVictoryFanfareOrchestrated.flac")
                .unwrap();

        sink.append(rodio::Decoder::new(BufReader::new(timer_sound_file)).unwrap());

        TimerState::PlayingTimerSound(clock_value, stream, sink)
    }

    fn stop_timer(&mut self) {
        if let TimerState::PlayingTimerSound(clock_value, _stream, sink) = self {
            sink.stop();
            *self = TimerState::SetTimer(*clock_value);
        }
    }
}

pub struct Timer {
    timer_state: TimerState,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            timer_state: TimerState::SetTimer(clock::ClockValue::default()),
        }
    }
}

impl eframe::App for Timer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}

#[allow(clippy::cast_possible_truncation)]
impl Timer {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;
        let mut style: egui::Style = (*ctx.style()).clone();

        // Set the font size for body and button text
        style
            .text_styles
            .get_mut(&egui::TextStyle::Body)
            .unwrap()
            .size = 20.0;
        style
            .text_styles
            .get_mut(&egui::TextStyle::Button)
            .unwrap()
            .size = 20.0;

        ctx.set_style(style);
        Self::default()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        match &mut self.timer_state {
            TimerState::SetTimer(clock_value) => {
                clock_value.ui_hms_input(ui);
                if ui.button("Start").clicked() {
                    self.timer_state = TimerState::start_countdown(*clock_value);
                }
                if ui.button("Reset").clicked() {
                    self.timer_state = TimerState::set_timer(clock::ClockValue::default());
                }
            }
            TimerState::Countdown(start_time, clock_value) => {
                // .5 seconds in nanoseconds
                // Make sure the window is refershing less than once a second so the second
                // countdown looks smooth
                ui.ctx()
                    .request_repaint_after(Duration::new(0, 500_000_000));
                // Use miliseconds so the visual update is less likely to be choppy
                // due to rounding. Honestly didn't fully check to see if it works
                // that way but it makes sense that it would as these arent float types
                let elapsed = start_time.elapsed().as_millis() as i32;
                let remaining_ms = clock_value.to_seconds() * 1000 - elapsed;
                if remaining_ms <= 0 {
                    self.timer_state = TimerState::play_timer_sound(*clock_value);
                    return;
                }

                ui.label(time_left_as_str(remaining_ms / 1000));
                if ui.button("Stop").clicked() {
                    self.timer_state = TimerState::start_countdown(*clock_value);
                }
            }
            TimerState::PlayingTimerSound(_clock_value, _stream, _sink) => {
                ui.label("Times Up!");
                if ui.button("Stop").clicked() {
                    self.timer_state.stop_timer();
                }
            }
        }
    }
}

fn time_left_as_str(remaining_sec: i32) -> String {
    // Convert remaining_sec to hours, minutes, and seconds
    let hours = remaining_sec / 3600;
    let minutes = (remaining_sec % 3600) / 60;
    let seconds = remaining_sec % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
