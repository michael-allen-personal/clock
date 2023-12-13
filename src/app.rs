use std::time::Instant;

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
        ui_counter(ui, &mut self.hour, Some("Hours:   ".to_string()));
        ui.end_row();
        ui_counter(ui, &mut self.min, Some("Minutes: ".to_string()));
        ui.end_row();
        ui_counter(ui, &mut self.sec, Some("Seconds: ".to_string()));
        ui.end_row();
    }
}

enum CountdownState {
    Active(Instant, ClockValue),
    Inactive(ClockValue),
}

pub struct ClockApp {
    countdown_state: CountdownState,
}

impl Default for ClockApp {
    fn default() -> Self {
        Self {
            countdown_state: CountdownState::Inactive(ClockValue::default()),
        }
    }
}

impl ClockApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match &mut self.countdown_state {
            CountdownState::Active(start_time, clock_value) => {
                let elapsed = start_time.elapsed().as_secs() as i64;
                let remaining_sec = clock_value.to_seconds() - elapsed;
                if remaining_sec < 0 {
                    self.countdown_state = CountdownState::Inactive(ClockValue {
                        hour: clock_value.hour,
                        min: clock_value.min,
                        sec: clock_value.sec,
                    });
                    return;
                }

                ui.label(time_left_as_str(remaining_sec));
                if ui.button("Stop").clicked() {
                    self.countdown_state = CountdownState::Inactive(ClockValue {
                        hour: clock_value.hour,
                        min: clock_value.min,
                        sec: clock_value.sec,
                    });
                }
            }
            CountdownState::Inactive(clock_value) => {
                clock_value.hms_input(ui);
                if ui.button("Start").clicked() {
                    self.countdown_state = CountdownState::Active(
                        Instant::now(),
                        ClockValue {
                            hour: clock_value.hour,
                            min: clock_value.min,
                            sec: clock_value.sec,
                        },
                    );
                }
            }
        });
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
