mod gamma;
mod log_transform;

use crate::{pipeline::Transform, widgets::transforms_panel::TransformPanelCache};
use uuid::Uuid;

pub fn show_transform_controls(
    ui: &mut egui::Ui,
    step_id: Uuid,
    transform: &mut Transform,
    cache: &mut TransformPanelCache,
    changed: &mut bool,
) {
    match transform {
        Transform::Negative => {
            ui.label("Параметры отсутствуют");
        }
        Transform::GammaCorrection(gamma_data) => {
            gamma::show_gamma_controls(ui, step_id, gamma_data, cache, changed);
        }
        Transform::LogTransform(log_transform_data) => {
            log_transform::show_log_transform_controls(
                ui,
                step_id,
                log_transform_data,
                cache,
                changed,
            );
        }
    }
}
