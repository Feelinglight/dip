#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod config;
mod errors;
mod widgets;

pub use app::DipPlotsApp;

pub use widgets::zoom_texture::ZoomTexture;
