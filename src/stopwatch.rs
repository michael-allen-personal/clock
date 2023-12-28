// use crate::clock;

enum StopwatchState {
    Counting,
    Stopped,
}

impl StopwatchState {
    pub fn start_counting() -> Self {
        StopwatchState::Counting
    }

    pub fn stop_counting() -> Self {
        StopwatchState::Stopped
    }
}

pub struct Stopwatch {
    stopwatch_state: StopwatchState,
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self {
            stopwatch_state: StopwatchState::Stopped,
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
