// #![warn(clippy::all, rust_2018_idioms)]
use std::path::PathBuf;

mod app;
mod config;
mod errors;
mod widgets;

pub use app::DipPlotsApp;

pub use widgets::zoom_texture::ZoomTexture;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        persistence_path: Some(PathBuf::from("./plots.ron")),
        ..Default::default()
    };
    eframe::run_native(
        "DIP visualizer",
        native_options,
        Box::new(|cc| Ok(Box::new(DipPlotsApp::new(cc)))),
    )
}
