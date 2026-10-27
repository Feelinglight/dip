use egui_plot::{Legend, Line, Plot, PlotPoints};
use intensity::gamma_correction::GammaCorrectionParams;
use log::error;
use uuid::Uuid;

use crate::widgets::transforms_panel::TransformPanelCache;

const PLOT_WIDTH: f32 = 200.;
const PLOT_HEIGHT: f32 = 200.;
const MIN_SLIDER_WIDTH: f32 = 50.;

fn slider_internal_to_gamma(internal: f64, gamma_max: f64) -> f64 {
    if internal <= 1. {
        internal
    } else {
        1. + (internal - 1.).powi(2) * gamma_max
    }
}

fn gamma_to_slider_internal(gamma: f64, gamma_max: f64) -> f64 {
    if gamma <= 1. {
        gamma
    } else {
        1. + ((gamma - 1.) / gamma_max).sqrt()
    }
}

fn new_gamma_correction_params(constant: f64, gamma: f64) -> GammaCorrectionParams {
    match GammaCorrectionParams::try_new(constant, gamma) {
        Ok(params) => params,
        Err(err) => {
            error!("Ошибка создания параметров гамма коррекции: {err}");
            GammaCorrectionParams::default()
        }
    }
}

pub fn show_gamma_controls(
    ui: &mut egui::Ui,
    step_id: Uuid,
    gamma_data: &mut GammaCorrectionParams,
    cache: &mut TransformPanelCache,
    changed: &mut bool,
) {
    let mut gamma = gamma_data.gamma();
    let mut constant = gamma_data.constant();

    let remaining_space = ui.available_width();
    ui.spacing_mut().slider_width = MIN_SLIDER_WIDTH.max(remaining_space - PLOT_WIDTH - 80.);

    let gamma_max = 29.;
    // Значение gamma внутри слайдера
    let mut internal_gamma = gamma_to_slider_internal(gamma, gamma_max);

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label("Гамма:");
            if ui
                .add(
                    egui::Slider::new(&mut internal_gamma, 0.001..=2.0).custom_formatter(
                        |value, _| format!("{:.2}", slider_internal_to_gamma(value, gamma_max)),
                    ),
                )
                .changed()
            {
                gamma = slider_internal_to_gamma(internal_gamma, gamma_max);
                *gamma_data = new_gamma_correction_params(constant, gamma);
                *changed = true;
            }

            ui.label("Константа:");
            if ui
                .add(egui::Slider::new(&mut constant, 1.0..=15.))
                .changed()
            {
                *gamma_data = new_gamma_correction_params(constant, gamma);
                *changed = true;
            }

            if ui.button("Сбросить").clicked() {
                gamma_data.reset();
                *changed = true;
            }
        });

        Plot::new("TransformsPanel::GammaCorrectionPlot")
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
                    PlotPoints::Borrowed(cache.gamma_points(step_id, gamma_data)),
                ));
            });
    });
}
