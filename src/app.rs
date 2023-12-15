use std::io::BufReader;
use std::time::{Duration, Instant};

use rodio::{OutputStream, Sink};

#[derive(Clone, Copy, Default)]
struct ClockValue {
    hour: i32,
    min: i32,
    sec: i32,
}

impl ClockValue {
    pub fn to_seconds(self) -> i32 {
        self.hour * 60 * 60 + self.min * 60 + self.sec
    }

    fn ui_hms_input(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |columns| {
            columns[0].label("Hours: ");
            ui_time_counter(&mut columns[1], &mut self.hour);
            columns[0].end_row();
            columns[1].end_row();

            columns[0].label("Minutes: ");
            ui_time_counter(&mut columns[1], &mut self.min);
            columns[0].end_row();
            columns[1].end_row();

            columns[0].label("Seconds: ");
            ui_time_counter(&mut columns[1], &mut self.sec);
            columns[0].end_row();
            columns[1].end_row();
        });
    }
}

enum AlarmState {
    SetAlarm(ClockValue),
    Countdown(Instant, ClockValue),
    PlayingAlarm(ClockValue, OutputStream, Sink),
}

impl AlarmState {
    fn set_alarm(clock_value: ClockValue) -> Self {
        AlarmState::SetAlarm(clock_value)
    }

    fn start_countdown(clock_value: ClockValue) -> Self {
        AlarmState::Countdown(Instant::now(), clock_value)
    }

    fn play_alarm(clock_value: ClockValue) -> Self {
        // get the default device every time, as sometimes I will change
        // output while the clock is still active. I would want the alarm
        // to play out of whatever the current output device is when it
        // goes off
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        let alarm_sound_file =
            std::fs::File::open("assets/FinalFantasyVictoryFanfareOrchestrated.flac").unwrap();

        sink.append(rodio::Decoder::new(BufReader::new(alarm_sound_file)).unwrap());

        AlarmState::PlayingAlarm(clock_value, stream, sink)
    }

    fn stop_alarm(&mut self) {
        if let AlarmState::PlayingAlarm(clock_value, _stream, sink) = self {
            sink.stop();
            *self = AlarmState::SetAlarm(*clock_value);
        }
    }
}

pub struct AlarmClock {
    alarm_state: AlarmState,
}

impl Default for AlarmClock {
    fn default() -> Self {
        Self {
            alarm_state: AlarmState::SetAlarm(ClockValue::default()),
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
impl eframe::App for AlarmClock {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match &mut self.alarm_state {
            AlarmState::SetAlarm(clock_value) => {
                clock_value.ui_hms_input(ui);
                if ui.button("Start").clicked() {
                    self.alarm_state = AlarmState::start_countdown(*clock_value);
                }
                if ui.button("Reset").clicked() {
                    self.alarm_state = AlarmState::set_alarm(ClockValue::default());
                }
            }
            AlarmState::Countdown(start_time, clock_value) => {
                // .75 seconds in nanoseconds
                // Make sure the window is refershing less than once a second the second
                // countdown looks smooth
                ctx.request_repaint_after(Duration::new(0, 500_000_000));
                // Use miliseconds so the visual update is less likely to be choppy
                // due to rounding. Honestly didn't fully check to see if it works
                // that way but it makes sense that it would as these arent float types
                let elapsed = start_time.elapsed().as_millis() as i32;
                let remaining_ms = clock_value.to_seconds() * 1000 - elapsed;
                if remaining_ms <= 0 {
                    self.alarm_state = AlarmState::play_alarm(*clock_value);
                    return;
                }

                ui.label(time_left_as_str(remaining_ms / 1000));
                if ui.button("Stop").clicked() {
                    self.alarm_state = AlarmState::start_countdown(*clock_value);
                }
            }
            AlarmState::PlayingAlarm(_clock_value, _stream, _sink) => {
                ui.label("Times Up!");
                if ui.button("Stop").clicked() {
                    self.alarm_state.stop_alarm();
                }
            }
        });
    }
}

impl AlarmClock {
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
}

fn ui_time_counter(ui: &mut egui::Ui, counter: &mut i32) {
    // This component ensures the counter value is
    // between 0 and 59, as minute and second time
    // values are between those numbers. Hours are not,
    // but I have not added days for it to roll over
    // into, and at a 59 hour alarm you are better off
    // using a calendar app anyways.
    // The buttons and label are on the same row.
    ui.horizontal(|ui| {
        if ui.button("−").clicked() {
            if counter.is_positive() {
                *counter -= 1;
            } else {
                *counter = 59;
            }
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            if *counter < 59 {
                *counter += 1;
            } else {
                *counter = 0;
            }
        }
    });
}

fn time_left_as_str(remaining_sec: i32) -> String {
    // Convert remaining_sec to hours, minutes, and seconds
    let hours = remaining_sec / 3600;
    let minutes = (remaining_sec % 3600) / 60;
    let seconds = remaining_sec % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
