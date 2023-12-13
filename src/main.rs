mod app;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Clock",
        native_options,
        Box::new(|cc| Box::new(app::ClockApp::new(cc))),
    )
}
