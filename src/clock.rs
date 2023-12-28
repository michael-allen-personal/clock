#[derive(Clone, Copy, Default)]
pub struct ClockValue {
    hour: i32,
    min: i32,
    sec: i32,
}

impl ClockValue {
    pub fn to_seconds(self) -> i32 {
        self.hour * 60 * 60 + self.min * 60 + self.sec
    }

    pub fn ui_hms_input(&mut self, ui: &mut egui::Ui) {
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

fn ui_time_counter(ui: &mut egui::Ui, counter: &mut i32) {
    // This component ensures the counter value is
    // between 0 and 59, as minute and second time
    // values are between those numbers. Hours are not,
    // but I have not added days for it to roll over
    // into, and at a 59 hour timer you are better off
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
