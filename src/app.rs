use std::time::Instant;

pub struct ClockApp {
    hour: i64,
    min: i64,
    sec: i64,
    countdown_active: bool,
    start_time: Instant,
}

impl Default for ClockApp {
    fn default() -> Self {
        Self {
            hour: 0,
            min: 0,
            sec: 0,
            countdown_active: false,
            start_time: Instant::now(),
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

    fn hms_input(&mut self, ui: &mut egui::Ui) {
        ui_counter(ui, &mut self.hour, Some("Hours:   ".to_string()));
        ui.end_row();
        ui_counter(ui, &mut self.min, Some("Minutes: ".to_string()));
        ui.end_row();
        ui_counter(ui, &mut self.sec, Some("Seconds: ".to_string()));
        ui.end_row();
    }
}

impl eframe::App for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.countdown_active {
                let elapsed = self.start_time.elapsed().as_secs() as i64;
                let total_seconds = self.hour * 60 * 60 + self.min * 60 + self.sec;
                let remaining_sec = total_seconds - elapsed;
                if remaining_sec < 0 {
                    self.countdown_active = false;
                    return;
                }

                // Convert remaining_sec to hours, minutes, and seconds
                let hours = remaining_sec / 3600;
                let minutes = (remaining_sec % 3600) / 60;
                let seconds = remaining_sec % 60;

                ui.label(format!("{:02}:{:02}:{:02}", hours, minutes, seconds));
                if ui.button("Stop").clicked() {
                    self.countdown_active = false;
                }
            } else {
                self.hms_input(ui);
                if ui.button("Start").clicked() {
                    self.countdown_active = true;
                    self.start_time = Instant::now();
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
