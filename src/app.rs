use std::io::BufReader;
use std::time::{Duration, Instant};

use rodio::{OutputStream, Sink};

struct ClockValue {
    hour: i64,
    min: i64,
    sec: i64,
}

impl Default for ClockValue {
    fn default() -> Self {
        Self {
            hour: 0,
            min: 0,
            sec: 0,
        }
    }
}

impl ClockValue {
    pub fn to_seconds(&self) -> i64 {
        self.hour * 60 * 60 + self.min * 60 + self.sec
    }

    fn hms_input(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |columns| {
            columns[0].label("Hours: ");
            columns[0].end_row();
            columns[0].label("Minutes: ");
            columns[0].end_row();
            columns[0].label("Seconds: ");
            columns[0].end_row();
            ui_counter(&mut columns[1], &mut self.hour, None);
            columns[1].end_row();
            ui_counter(&mut columns[1], &mut self.min, None);
            columns[1].end_row();
            ui_counter(&mut columns[1], &mut self.sec, None);
            columns[1].end_row();
        });
    }
}

enum AlarmState {
    Countdown(Instant, ClockValue),
    SetAlarm(ClockValue),
    PlayingAlarm(ClockValue, OutputStream, Sink),
}

pub struct ClockApp {
    alarm_state: AlarmState,
}

impl Default for ClockApp {
    fn default() -> Self {
        Self {
            alarm_state: AlarmState::SetAlarm(ClockValue::default()),
        }
    }
}

impl eframe::App for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match &mut self.alarm_state {
            AlarmState::SetAlarm(clock_value) => {
                clock_value.hms_input(ui);
                if ui.button("Start").clicked() {
                    self.alarm_state = AlarmState::Countdown(
                        Instant::now(),
                        ClockValue {
                            hour: clock_value.hour,
                            min: clock_value.min,
                            sec: clock_value.sec,
                        },
                    );
                }
                if ui.button("Reset").clicked() {
                    self.alarm_state = AlarmState::SetAlarm(ClockValue::default());
                }
            }
            AlarmState::Countdown(start_time, clock_value) => {
                // .75 seconds in nanoseconds
                ctx.request_repaint_after(Duration::new(0, 750000000));
                let elapsed = start_time.elapsed().as_millis() as i64;
                let remaining_ms = clock_value.to_seconds() * 1000 - elapsed;
                if remaining_ms <= 0 {
                    let (stream, handle) = rodio::OutputStream::try_default().unwrap();
                    let sink = rodio::Sink::try_new(&handle).unwrap();

                    let alarm_sound_file =
                        std::fs::File::open("assets/FinalFantasyVictoryFanfareOrchestrated.flac")
                            .unwrap();
                    sink.append(rodio::Decoder::new(BufReader::new(alarm_sound_file)).unwrap());
                    self.alarm_state = AlarmState::PlayingAlarm(
                        ClockValue {
                            hour: clock_value.hour,
                            min: clock_value.min,
                            sec: clock_value.sec,
                        },
                        stream,
                        sink,
                    );
                    return;
                }

                ui.label(time_left_as_str(i64::from(remaining_ms / 1000)));
                if ui.button("Stop").clicked() {
                    self.alarm_state = AlarmState::SetAlarm(ClockValue {
                        hour: clock_value.hour,
                        min: clock_value.min,
                        sec: clock_value.sec,
                    });
                }
            }
            AlarmState::PlayingAlarm(clock_value, _stream, sink) => {
                if ui.button("Stop").clicked() {
                    sink.stop();
                    self.alarm_state = AlarmState::SetAlarm(ClockValue {
                        hour: clock_value.hour,
                        min: clock_value.min,
                        sec: clock_value.sec,
                    });
                }
            }
        });
    }
}

impl ClockApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let ctx = &cc.egui_ctx;
        let mut style: egui::Style = (*ctx.style()).clone();

        // Set the font size for body text
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

fn ui_counter(ui: &mut egui::Ui, counter: &mut i64, label_text: Option<String>) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui| {
        if label_text.is_some() {
            ui.label(label_text.unwrap());
        }
        if ui.button("−").clicked() && counter.is_positive() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}

fn time_left_as_str(remaining_sec: i64) -> String {
    // Convert remaining_sec to hours, minutes, and seconds
    let hours = remaining_sec / 3600;
    let minutes = (remaining_sec % 3600) / 60;
    let seconds = remaining_sec % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
