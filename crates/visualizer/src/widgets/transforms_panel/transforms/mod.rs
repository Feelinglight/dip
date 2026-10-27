mod gamma_ui;
mod log_transform_ui;

use crate::{
    pipeline::{Transform, TransformKind},
    widgets::transforms_panel::TransformPanelCache,
};
use intensity::{
    gamma_correction::gamma_correct_single, log_transform::log_transform_single,
    negative::negative_single,
};
use uuid::Uuid;

pub fn transform_single(value: f64, transform: Transform) -> f64 {
    match transform {
        Transform::Negative => negative_single(value),
        Transform::GammaCorrection(params) => gamma_correct_single(value, &params),
        Transform::LogTransform(params) => log_transform_single(value, &params),
    }
}

pub const fn transform_name(transform_kind: TransformKind) -> &'static str {
    match transform_kind {
        TransformKind::Negative => "Негатив",
        TransformKind::GammaCorrection => "Гамма-коррекция",
        TransformKind::LogTransform => "Логарифм. преобраз.",
    }
}

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
            gamma_ui::show_gamma_controls(ui, step_id, gamma_data, cache, changed);
        }
        Transform::LogTransform(log_transform_data) => {
            log_transform_ui::show_log_transform_controls(
                ui,
                step_id,
                log_transform_data,
                cache,
                changed,
            );
        }
    }
}
