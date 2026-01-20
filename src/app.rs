use std::path::Path;

use egui_plot::{Bar, BarChart, Legend, Plot};

use crate::{ZoomTexture, config::AppConfig};

pub struct DipPlotsApp {
    config: AppConfig,
}

impl DipPlotsApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut config: AppConfig = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        config
            .zt_state
            .load_image(&cc.egui_ctx, Path::new(&config.image_path_edit_text), false);

        Self { config }
    }
}

impl eframe::App for DipPlotsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Diplib visualizer");

            ui.separator();

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.config.image_path_edit_text);

                if ui.button("Открыть изображение").clicked() {
                    self.config.zt_state.load_image(
                        ui.ctx(),
                        Path::new(&self.config.image_path_edit_text),
                        true,
                    );
                }

                if ui.button("Сбросить расположение").clicked() {
                    self.config.zt_state.reset_parameters();
                }

                ui.separator();
                ui.checkbox(&mut self.config.hist_enable, "Показать гистограмму");
            });

            ui.separator();

            if self.config.hist_enable {
                egui::SidePanel::right("plots").show_inside(ui, |ui| {
                    ui.vertical(|ui| {
                        histogram(ui);
                    });
                });
            }

            let zoom_widget = ZoomTexture::new(&mut self.config.zt_state, ui.available_size());
            ui.add(zoom_widget);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }
}

fn histogram(ui: &mut egui::Ui) {
    let chart = BarChart::new(
        "Normal Distribution",
        (-395..=395)
            .step_by(1)
            .map(|x| x as f64 * 0.01)
            .map(|x| {
                (
                    x,
                    (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt(),
                )
            })
            .map(|(x, f)| Bar::new(x, f * 10.0).width(0.1))
            .collect(),
    )
    .color(egui::Color32::LIGHT_BLUE);

    Plot::new("Normal Distribution Demo")
        // .(Vec2 { x: 400., y: 0. })
        .legend(Legend::default())
        .clamp_grid(true)
        .allow_zoom(egui::Vec2b::new(true, true))
        .allow_drag(egui::Vec2b::new(true, true))
        .allow_scroll(egui::Vec2b::new(false, false))
        .show(ui, |plot_ui| plot_ui.bar_chart(chart));
}
