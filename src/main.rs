use egui::Vec2;

mod app;
mod clock;
mod stopwatch;
mod timer;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            max_inner_size: Some(Vec2::new(480.0, 200.0)),
            min_inner_size: Some(Vec2::new(240.0, 150.0)),
            maximize_button: Some(false),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "Clock",
        native_options,
        Box::new(|cc| Box::new(app::Application::new(cc))),
    )
}
