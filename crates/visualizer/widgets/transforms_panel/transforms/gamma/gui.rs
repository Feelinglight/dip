use egui_plot::{Legend, Line, Plot, PlotPoints};

use super::data::GammaCorrectionData;

fn slider_internal_to_gamma(internal: f64, gamma_max: f64) -> f64 {
    if internal <= 1. {
        internal
    } else {
        1. + (internal - 1.) * gamma_max
    }
}

fn gamma_to_slider_internal(gamma: f64, gamma_max: f64) -> f64 {
    if gamma <= 1. {
        gamma
    } else {
        1. + (gamma - 1.) / gamma_max
    }
}

pub fn show_gamma_controls(ui: &mut egui::Ui, gamma_data: &mut GammaCorrectionData) {
    let mut gamma = gamma_data.gamma();
    let mut constant = gamma_data.constant();

    let plot_width = 200.;
    let plot_height = 200.;

    let remaining_space = ui.available_width();
    let min_slider_width = 50_f32;
    ui.spacing_mut().slider_width = min_slider_width.max(remaining_space - plot_width - 80.);

    let gamma_max = 99.;
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
                gamma_data.set_parameters(constant, gamma);
            }

            ui.label("Константа:");
            if ui
                .add(egui::Slider::new(&mut constant, 1.0..=100.))
                .changed()
            {
                gamma_data.set_parameters(constant, gamma);
            }

            if ui.button("Сбросить").clicked() {
                gamma_data.reset_parameters();
            }
        });

        Plot::new("TransformsPanel::GammaCorrectionPlot")
            .legend(Legend::default())
            .width(plot_width)
            .height(plot_height)
            .clamp_grid(true)
            .allow_zoom(egui::Vec2b::new(false, false))
            .allow_drag(egui::Vec2b::new(false, false))
            .allow_scroll(egui::Vec2b::new(false, false))
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new(
                    "График",
                    PlotPoints::Borrowed(gamma_data.plot_points()),
                ));
            });
    });
}
