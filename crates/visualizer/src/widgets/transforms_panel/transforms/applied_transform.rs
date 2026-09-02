use crate::{pipeline::Transform, widgets::transforms_panel::TransformEditorCache};
use uuid::Uuid;

pub use super::gamma::show_gamma_controls;
pub use super::log_transform::show_log_transform_controls;

pub fn show_transform_controls(
    ui: &mut egui::Ui,
    step_id: Uuid,
    transform: &mut Transform,
    cache: &mut TransformEditorCache,
    changed: &mut bool,
) {
    match transform {
        Transform::Negative => {
            ui.label("Параметры отсутствуют");
        }
        Transform::GammaCorrection(gamma_data) => {
            show_gamma_controls(ui, step_id, gamma_data, cache, changed);
        }
        Transform::LogTransform(log_transform_data) => {
            show_log_transform_controls(ui, step_id, log_transform_data, cache, changed);
        }
    }
}
