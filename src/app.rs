use egui::{ColorImage, SidePanel, TextureHandle, Vec2};
use image::ImageReader;

use crate::{ZoomWidget, config::AppConfig, widgets::zoom_image::ZoomWidgetConfig};

pub struct DipPlotsApp {
    config: AppConfig,
    label: String,
    value: f32,

    zw_config: ZoomWidgetConfig,
}

impl DipPlotsApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config: AppConfig = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let image = ImageReader::open("/home/dmitry/data/develop/cv/plots/image.jpg")
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        let image_samples = image.to_rgba8();

        let ci = ColorImage::from_rgba_unmultiplied(
            [image.width() as usize, image.height() as usize],
            image_samples.as_flat_samples().as_slice(),
        );

        Self {
            config: config,
            label: "Hello world".to_owned(),
            value: 2.7,
            zw_config: ZoomWidgetConfig::new(cc.egui_ctx.load_texture(
                "dip",
                ci,
                egui::TextureOptions::LINEAR,
            )),
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
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Diplib visualizer");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.separator();

            egui::SidePanel::right("plots")
                // .default_width(self.panel_response)
                .show_inside(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
                        if ui.button("Increment").clicked() {
                            self.value += 1.0;
                        }
                        // Чтобы ресайз не сбрасывался
                        ui.allocate_space(ui.available_size());
                    });
                });

            let zoom_widget = ZoomWidget::new(&mut self.zw_config, ui.available_size());
            ui.add(zoom_widget);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }
}

fn calc_scaled_image_size(ui: &mut egui::Ui, origin_size: Vec2) -> Vec2 {
    // available area within the central panel
    let avail = ui.available_size();

    // compute scale that fits the image into the available rect while preserving aspect ratio
    // (if you want to avoid enlarging images beyond their native size, add `.min(1.0)`)
    let scale = (avail.x / origin_size.x).min(avail.y / origin_size.y);

    // handle degenerate cases (zero / infinite)
    let scale = if !scale.is_finite() || scale <= 0.0 {
        1.0
    } else {
        scale
    };

    egui::vec2(origin_size.x * scale, origin_size.y * scale)
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
