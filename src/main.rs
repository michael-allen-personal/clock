use egui::Vec2;

mod clock;
mod timer;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            max_inner_size: Some(Vec2::new(190.0, 150.0)),
            min_inner_size: Some(Vec2::new(190.0, 150.0)),
            maximize_button: Some(false),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "Clock",
        native_options,
        Box::new(|cc| Box::new(timer::Timer::new(cc))),
    )
}
