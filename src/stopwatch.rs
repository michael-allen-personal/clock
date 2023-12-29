use std::time::Instant;

use crate::app;
use crate::clock;

enum StopwatchState {
    Counting(Instant, clock::TimeDisplay),
    Stopped(clock::TimeDisplay),
}

impl StopwatchState {
    pub fn start_counting(clock_value: clock::TimeDisplay) -> Self {
        StopwatchState::Counting(Instant::now(), clock_value)
    }

    pub fn stop_counting(clock_value: clock::TimeDisplay) -> Self {
        StopwatchState::Stopped(clock_value)
    }

    pub fn reset_counter() -> Self {
        StopwatchState::Stopped(clock::TimeDisplay::default())
    }
}

pub struct Stopwatch {
    stopwatch_state: StopwatchState,
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self {
            stopwatch_state: StopwatchState::Stopped(clock::TimeDisplay::default()),
        }
    }
}

impl app::Ui for Stopwatch {
    fn ui(&mut self, ui: &mut egui::Ui) {
        match &mut self.stopwatch_state {
            StopwatchState::Stopped(clock_value) => {
                ui.label(&clock_value.to_string());
                if ui.button("Start").clicked() {
                    self.stopwatch_state = StopwatchState::start_counting(*clock_value);
                }
                if ui.button("Reset").clicked() {
                    self.stopwatch_state = StopwatchState::reset_counter();
                }
            }
            StopwatchState::Counting(start_time, clock_value) => {
                ui.ctx().request_repaint();
                let elapsed_clock_value = *clock_value + start_time.elapsed();
                ui.label(elapsed_clock_value.to_string());
                if ui.button("Pause").clicked() {
                    self.stopwatch_state = StopwatchState::stop_counting(elapsed_clock_value);
                }
            }
        }
    }
}
