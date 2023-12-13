pub struct ClockApp {
    hour: i32,
    min: i32,
    sec: i32,
}

impl Default for ClockApp {
    fn default() -> Self {
        Self {
            hour: 0,
            min: 0,
            sec: 0,
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
            ui.heading("Hello World!");
            self.hms_input(ui)
        });
    }
}

fn ui_counter(ui: &mut egui::Ui, counter: &mut i32, label_text: Option<String>) {
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
