use std::path::Path;

use egui_file_dialog::FileDialog;
use egui_plot::{Bar, BarChart, Legend, Plot};
use image::{GrayImage, ImageReader};

use crate::{ZoomTexture, config::AppConfig, errors::LoadImageError};

pub struct DipPlotsApp {
    config: AppConfig,
    file_dialog: FileDialog,

    image: Option<GrayImage>,
}

impl DipPlotsApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut config: AppConfig = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let image = load_image(Path::new(&config.image_path_edit_text));
        config
            .zt_state
            .set_texture(&cc.egui_ctx, image.as_ref(), false);

        Self {
            config,
            file_dialog: FileDialog::new(),
            image: image.ok(),
        }
    }

    fn reload_image(&mut self, ctx: &egui::Context) {
        let image = load_image(Path::new(&self.config.image_path_edit_text));
        self.config.zt_state.set_texture(ctx, image.as_ref(), false);
        self.image = image.ok();
    }

    fn update_zoom_texture(&mut self, ctx: &egui::Context) {
        if let Some(img) = &self.image {
            self.config.zt_state.set_texture(ctx, Ok(img), false);
        }
    }

    fn test_function(&mut self, ui: &mut egui::Ui) {
        if let Some(img) = &mut self.image {
            for sample in img.as_flat_samples_mut().samples.iter_mut() {
                *sample = sample.saturating_add(10);
            }
            self.update_zoom_texture(ui.ctx());
        }
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
            ui.horizontal(|ui| {
                self.pick_file(ctx, ui);

                if ui.button("Сбросить расположение").clicked() {
                    self.config.zt_state.reset_parameters();
                }

                if ui.button("Тест").clicked() {
                    self.test_function(ui);
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.checkbox(&mut self.config.hist_enable, "Показать гистограмму");
                });
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

impl DipPlotsApp {
    fn pick_file(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if ui.button("Открыть изображение").clicked() {
            self.file_dialog.pick_file();
        }
        self.file_dialog.update(ctx);

        if let Some(path) = self.file_dialog.take_picked()
            && let Some(path_str) = path.to_str()
        {
            self.config.image_path_edit_text = String::from(path_str);
            self.reload_image(ui.ctx());
        }

        ui.text_edit_singleline(&mut self.config.image_path_edit_text);
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

fn load_image(path: &Path) -> Result<GrayImage, LoadImageError> {
    Ok(ImageReader::open(path)?
        .with_guessed_format()?
        .decode()?
        .to_luma8())
}
