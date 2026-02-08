use crate::{config::AppConfig, widgets::image_hist::ImageHist};
use egui_file_dialog::FileDialog;

pub struct DipPlotsApp {
    config: AppConfig,
    file_dialog: FileDialog,
}

impl DipPlotsApp {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut config: AppConfig = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            AppConfig::default()
        };

        config.image_states.init(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);
        catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);

        Self {
            config,
            file_dialog: FileDialog::new(),
        }
    }
}

impl eframe::App for DipPlotsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut unique_id = 0usize;
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
            let mut image_hist = ImageHist::new(unique_id, &mut self.config.image_states);
            ui.add(&mut image_hist);
            unique_id += 1;
            if image_hist.open_image_requested() {
                self.file_dialog.pick_file();
            }
            self.file_dialog.update(ctx);
            if let Some(path) = self.file_dialog.take_picked() {
                self.config.image_states.load_image(ui.ctx(), &path);
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }
}
