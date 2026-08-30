use std::path::Path;

use eframe::Frame;
use egui_dock::{DockArea, DockState};
use log::error;

use crate::{
    config::AppConfig,
    image_picker::{ImagePicker, PickedImage},
    theme,
    widgets::{image_hist::ImageHistState, tabs},
};

pub struct DipPlotsApp {
    config: AppConfig,
    tree: DockState<tabs::ImageHistTab>,
    image_picker: ImagePicker,
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

        let mut tree: DockState<tabs::ImageHistTab> = serde_json::from_str(&config.tabs_state)
            .ok()
            .unwrap_or_else(|| DockState::new(vec![tabs::ImageHistTab::default()]));

        for (_, image_hist_tab) in tree.iter_all_tabs_mut() {
            image_hist_tab.state.restore();
        }

        let image_picker = ImagePicker::new(config.last_image_path.as_ref());

        Self {
            config,
            tree,
            image_picker,
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
            if ui.button("Вот так добавлять кнопки над DockArea").clicked()
            {
                println!("Вот так");
            }
        });

        let tabs_count = self.tree.iter_all_tabs().count();

        DockArea::new(&mut self.tree)
            .style(egui_dock::Style::from_egui(ui.style().as_ref()))
            .show_add_buttons(true)
            .show_tab_name_on_hover(true)
            .show_inside(
                ui,
                &mut tabs::TabViewer::new(tabs_count, &mut |path| {
                    self.image_picker.open_pick_window(path);
                }),
            );

        self.image_picker.poll_picked_image(
            |PickedImage {
                 path,
                 bytes,
                 target_node: node_path,
             }| {
                match ImageHistState::from_memory(Path::new(&path), &bytes) {
                    Ok(image_hist_state) => {
                        self.tree.set_focused_node_and_surface(node_path);
                        self.tree
                            .push_to_focused_leaf(tabs::ImageHistTab::new(image_hist_state));
                    }
                    Err(err) => {
                        error!("Не удалось загрузить изображение по пути \"{path}\": {err}");
                    }
                }
            },
        );
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(tree_json) = serde_json::to_string(&self.tree) {
            self.config.tabs_state = tree_json;
        }
        self.config.last_image_path = self.image_picker.get_pick_path().to_string();
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }
}
