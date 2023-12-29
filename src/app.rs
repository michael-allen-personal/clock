use crate::stopwatch;
use crate::timer;

pub trait Ui {
    fn ui(&mut self, _ui: &mut egui::Ui);
}

enum ApplicationView {
    Timer(timer::Timer),
    Stopwatch(stopwatch::Stopwatch),
}

pub struct Application {
    application_view: ApplicationView,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            application_view: ApplicationView::Timer(timer::Timer::default()),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("Feature Selector").show(ctx, |ui| self.feature_selector(ui));
        egui::CentralPanel::default().show(ctx, |ui| match &mut self.application_view {
            ApplicationView::Timer(timer) => timer.ui(ui),
            ApplicationView::Stopwatch(stopwatch) => stopwatch.ui(ui),
        });
    }
}

impl Application {
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

    fn feature_selector(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| match &mut self.application_view {
            ApplicationView::Timer(_timer) => {
                if ui.button("Stopwatch").clicked() {
                    self.application_view =
                        ApplicationView::Stopwatch(stopwatch::Stopwatch::default());
                }
            }
            ApplicationView::Stopwatch(_stopwatch) => {
                if ui.button("Timer").clicked() {
                    self.application_view = ApplicationView::Timer(timer::Timer::default());
                }
            }
        });
    }
}
