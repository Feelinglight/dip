use log::{error, warn};
use std::{path::Path, sync::mpsc};

use crate::{
    config::AppConfig,
    widgets::{
        image_hist::ImageHistState,
        tabs::{self, ImageHistTab},
    },
};
use egui_dock::{DockArea, DockState};

pub struct DipPlotsApp {
    config: AppConfig,
    tree: DockState<ImageHistTab>,
    filepath_tx: mpsc::Sender<(
        egui_dock::SurfaceIndex,
        egui_dock::NodeIndex,
        String,
        Vec<u8>,
    )>,
    filepath_rx: mpsc::Receiver<(
        egui_dock::SurfaceIndex,
        egui_dock::NodeIndex,
        String,
        Vec<u8>,
    )>,
}

impl DipPlotsApp {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut config: AppConfig = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            AppConfig::default()
        };

        config.image_states = vec![ImageHistState::default()];

        for state in &mut config.image_states {
            state.init(&cc.egui_ctx);
        }

        egui_extras::install_image_loaders(&cc.egui_ctx);
        catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);

        let tree = serde_json::from_str(&config.tabs_state)
            .ok()
            .unwrap_or_else(|| {
                let mut tree =
                    DockState::new(vec![ImageHistTab::default(), ImageHistTab::default()]);

                // let mut tree = DockState::new(vec!["tab1".to_owned(), "tab2".to_owned()]);
                //
                // let [a, b] = tree.main_surface_mut().split_left(
                //     NodeIndex::root(),
                //     0.3,
                //     vec!["tab3".to_owned()],
                // );
                // let [_, _] = tree
                //     .main_surface_mut()
                //     .split_below(a, 0.7, vec!["tab4".to_owned()]);
                // let [_, _] = tree
                //     .main_surface_mut()
                //     .split_below(b, 0.5, vec!["tab5".to_owned()]);

                tree
            });

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

        egui::TopBottomPanel::top("my_top_bar").show(ctx, |ui| {
            if ui.button("Test").clicked() {
                println!("test");
            }
        });

        DockArea::new(&mut self.tree)
            .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
            .show_add_buttons(true)
            .show_tab_name_on_hover(true)
            .show(ctx, &mut tabs::TabViewer::new(self.filepath_tx.clone()));

        match self.filepath_rx.try_recv() {
            Ok((surface_idx, node_idx, filepath, data)) => {
                match ImageHistState::from_memory(ctx, Path::new(&filepath), &data) {
                    Ok(image_hist_state) => {
                        self.tree
                            .set_focused_node_and_surface((surface_idx, node_idx));
                        self.tree
                            .push_to_focused_leaf(ImageHistTab::new(image_hist_state));
                    }
                    Err(err) => {
                        error!("Не удалось загрузить изображение по пути \"{filepath}\": {err}");
                    }
                }
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
