use std::fmt;
use std::fmt::Display;
use std::ops::Add;
use std::time::Duration;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct TimeDisplay {
    hour: i32,
    min: i32,
    sec: i32,
    ms: i32,
}

impl Add for TimeDisplay {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            hour: self.hour + rhs.hour,
            min: self.min + rhs.min,
            sec: self.sec + rhs.sec,
            ms: self.ms + rhs.ms,
        }
    }
}

impl Add<Duration> for TimeDisplay {
    type Output = Self;

    fn add(self, duration: Duration) -> Self {
        self + TimeDisplay::new_from_duration(duration)
    }
}

impl Display for TimeDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}:{:03}",
            self.hour, self.min, self.sec, self.ms
        )
    }
}

// i32 can hold almost 600 hours in ms, no realistic need to worry about overflow
#[allow(clippy::cast_possible_truncation)]
impl TimeDisplay {
    pub fn new_from_duration(duration: Duration) -> Self {
        let dur_ms = duration.as_millis() as i32;

        // There are 3,600,000 milliseconds in an hour
        let hour = dur_ms / 3_600_000;
        let remaining_after_hours = dur_ms % 3_600_000;

        // There are 60,000 milliseconds in a minute
        let min = remaining_after_hours / 60_000;
        let remaining_after_minutes = remaining_after_hours % 60_000;

        // There are 1,000 milliseconds in a second
        let sec = remaining_after_minutes / 1_000;
        let ms = remaining_after_minutes % 1_000;

        Self { hour, min, sec, ms }
    }

    pub fn as_sec_i32(self) -> i32 {
        self.hour * 60 * 60 + self.min * 60 + self.sec
    }

    //    pub fn as_ms_i32(self) -> i32 {
    //        self.as_sec_i32() * 1_000 + self.ms
    //    }

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
