mod app;
mod ui;
mod api;
mod config;
mod crypto;

use eframe::egui;
use app::PulseApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        min_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Pulse",
        options,
        Box::new(|cc| Box::new(PulseApp::new(cc))),
    )
} 