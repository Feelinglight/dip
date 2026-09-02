use egui_plot::{Legend, Line, Plot, PlotPoints};
use intensity::log_transform::LogTransformParams;
use uuid::Uuid;

use crate::widgets::transforms_panel::TransformEditorCache;

const PLOT_WIDTH: f32 = 200.;
const PLOT_HEIGHT: f32 = 200.;
const MIN_SLIDER_WIDTH: f32 = 50.;

fn slider_internal_to_log_base(internal: f64, log_base_max: f64) -> f64 {
    if internal <= 1. {
        internal
    } else {
        1. + (internal - 1.).powi(2) * log_base_max
    }
}

fn log_base_to_slider_internal(log_base: f64, log_base_max: f64) -> f64 {
    if log_base <= 1. {
        log_base
    } else {
        1. + ((log_base - 1.) / log_base_max).sqrt()
    }
}

// Корректирует значение основания логарифма так, чтобы оно не получилось равным нулю.
// Если предыдущее значение было меньше нуля , то добавляет к текущему небольшое значение
// Если больше, то отнимает
fn correct_log_base(current: f64, prev: f64) -> f64 {
    if (current - 1.).abs() < 0.01 {
        if prev < 0. {
            current + 0.01
        } else {
            current - 0.01
        }
    } else {
        current
    }
}

pub fn show_log_transform_controls(
    ui: &mut egui::Ui,
    step_id: Uuid,
    log_transform_data: &mut LogTransformParams,
    cache: &mut TransformEditorCache,
    changed: &mut bool,
) {
    let mut log_base = log_transform_data.log_base();
    let mut constant = log_transform_data.constant();

    let remaining_space = ui.available_width();
    ui.spacing_mut().slider_width = MIN_SLIDER_WIDTH.max(remaining_space - PLOT_WIDTH - 80.);

    let log_base_max = 99.;
    let mut internal_log_base = log_base_to_slider_internal(log_base, log_base_max);

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label("Основание логарифма:");
            if ui
                .add(
                    egui::Slider::new(&mut internal_log_base, 0.001..=2.0).custom_formatter(
                        |value, _| {
                            format!("{:.2}", slider_internal_to_log_base(value, log_base_max))
                        },
                    ),
                )
                .changed()
            {
                log_base = slider_internal_to_log_base(internal_log_base, log_base_max);
                log_base = correct_log_base(log_base, log_transform_data.log_base());

                *log_transform_data = LogTransformParams::new(constant, log_base);
                *changed = true;
            }

            ui.label("Константа:");
            if ui
                .add(egui::Slider::new(&mut constant, 1.0..=15.))
                .changed()
            {
                *log_transform_data = LogTransformParams::new(constant, log_base);
                *changed = true;
            }

            if ui.button("Сбросить").clicked() {
                log_transform_data.reset();
                *changed = true;
            }
        });

        Plot::new("TransformsPanel::LogTransformPlot")
            .legend(Legend::default())
            .width(PLOT_WIDTH)
            .height(PLOT_HEIGHT)
            .clamp_grid(true)
            .allow_zoom(egui::Vec2b::new(false, false))
            .allow_drag(egui::Vec2b::new(false, false))
            .allow_scroll(egui::Vec2b::new(false, false))
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new(
                    "График",
                    PlotPoints::Borrowed(cache.log_points(step_id, log_transform_data)),
                ));
            });
    });
}
