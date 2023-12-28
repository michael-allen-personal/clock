use std::time::{Duration, Instant};

use crate::clock;

enum StopwatchState {
    Counting(Instant, clock::ClockValue),
    Stopped(clock::ClockValue),
}

impl StopwatchState {
    pub fn start_counting(clock_value: clock::ClockValue) -> Self {
        StopwatchState::Counting(Instant::now(), clock_value)
    }

    pub fn stop_counting(clock_value: clock::ClockValue) -> Self {
        StopwatchState::Stopped(clock_value)
    }
}

pub struct Stopwatch {
    stopwatch_state: StopwatchState,
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self {
            stopwatch_state: StopwatchState::Stopped(clock::ClockValue::default()),
        }
    }
}

impl eframe::App for Stopwatch {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}

#[allow(clippy::cast_possible_truncation)]
impl Stopwatch {
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

    fn ui(&mut self, _ui: &mut egui::Ui) {
        match &mut self.stopwatch_state {
            StopwatchState::Stopped => todo!(),
            StopwatchState::Counting => todo!(),
        }
    }
}
