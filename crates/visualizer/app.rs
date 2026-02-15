use crate::{
    config::AppConfig,
    widgets::{image_hist::ImageHistState, tabs},
};
use egui_dock::{DockArea, DockState};
use egui_file_dialog::FileDialog;

pub struct DipPlotsApp {
    config: AppConfig,
    file_dialog: FileDialog,
    tree: DockState<ImageHistState>,
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

        for state in config.image_states.iter_mut() {
            state.init(&cc.egui_ctx);
        }
        egui_extras::install_image_loaders(&cc.egui_ctx);
        catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);

        let tree = serde_json::from_str(&config.tabs_state)
            .ok()
            .unwrap_or_else(|| {
                let mut tree =
                    DockState::new(vec![ImageHistState::default(), ImageHistState::default()]);

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

        Self {
            config,
            file_dialog: FileDialog::new(),
            tree,
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

        // egui::CentralPanel::default().show(ctx, |ui| {

        // egui::TopBottomPanel::top("my_top_bar").show(ctx, |ui| {
        // ui.button("Test");
        // });

        DockArea::new(&mut self.tree)
            .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tabs::TabViewer {});

        // unique_id += 1;
        // if image_hist.open_image_requested() {
        //     self.file_dialog.pick_file();
        // }
        // self.file_dialog.update(ctx);
        // if let Some(path) = self.file_dialog.take_picked() {
        //     self.config.image_states.load_image(ui.ctx(), &path);
        // }
        // });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(tree_json) = serde_json::to_string(&self.tree) {
            self.config.tabs_state = tree_json;
        }
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }
}
