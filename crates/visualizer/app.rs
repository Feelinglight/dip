use eframe::Frame;
use log::{error, warn};
use std::{path::Path, sync::mpsc};

use crate::{
    config::AppConfig,
    theme,
    widgets::{
        image_hist::ImageHistState,
        tabs::{self, ImageHistTab},
    },
};
use egui_dock::{DockArea, DockState};

pub struct DipPlotsApp {
    config: AppConfig,
    tree: DockState<ImageHistTab>,
    filepath_tx: mpsc::Sender<(egui_dock::NodePath, String, Vec<u8>)>,
    filepath_rx: mpsc::Receiver<(egui_dock::NodePath, String, Vec<u8>)>,
}

impl DipPlotsApp {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config: AppConfig = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            AppConfig::default()
        };
        theme::apply_theme(&cc.egui_ctx, theme::ThemeId::CatppuccinMocha);

        egui_extras::install_image_loaders(&cc.egui_ctx);

        let mut tree: DockState<ImageHistTab> = serde_json::from_str(&config.tabs_state)
            .ok()
            .unwrap_or_else(|| DockState::new(vec![ImageHistTab::default()]));

        for (_, image_hist_tab) in tree.iter_all_tabs_mut() {
            image_hist_tab.state.restore(&cc.egui_ctx);
        }

        let (filepath_tx, filepath_rx) = mpsc::channel();

        Self {
            config,
            tree,
            filepath_tx,
            filepath_rx,
        }
    }
}

impl eframe::App for DipPlotsApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
            });
        });

        egui::Panel::top("my_top_bar").show(ui, |ui| {
            if ui.button("Test").clicked() {
                println!("test");
            }
        });

        let tabs_count = self.tree.iter_all_tabs().count();

        DockArea::new(&mut self.tree)
            .style(egui_dock::Style::from_egui(ui.style().as_ref()))
            .show_add_buttons(true)
            .show_tab_name_on_hover(true)
            .show_inside(
                ui,
                &mut tabs::TabViewer::new(
                    tabs_count,
                    &self.config.last_image_folder_path,
                    self.filepath_tx.clone(),
                ),
            );

        match self.filepath_rx.try_recv() {
            Ok((node_path, filepath, data)) => {
                match ImageHistState::from_memory(ui, Path::new(&filepath), &data) {
                    Ok(image_hist_state) => {
                        self.tree.set_focused_node_and_surface(node_path);
                        self.tree
                            .push_to_focused_leaf(ImageHistTab::new(image_hist_state));
                    }
                    Err(err) => {
                        error!("Не удалось загрузить изображение по пути \"{filepath}\": {err}");
                    }
                }
                self.config.last_image_folder_path = filepath;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                warn!("Канал отключился до того как имя файла было принято");
            }
            Err(mpsc::TryRecvError::Empty) => {}
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(tree_json) = serde_json::to_string(&self.tree) {
            self.config.tabs_state = tree_json;
        }
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }
}
