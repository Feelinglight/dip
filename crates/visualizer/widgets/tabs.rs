use std::sync::mpsc;

use egui_dock::NodePath;
use log::warn;

use crate::widgets::image_hist::{ImageHist, ImageHistState};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ImageHistTab {
    pub id: egui::Id,
    pub state: ImageHistState,
}

impl Default for ImageHistTab {
    fn default() -> Self {
        Self {
            id: egui::Id::new(uuid::Uuid::new_v4()),
            state: ImageHistState::default(),
        }
    }
}

impl ImageHistTab {
    pub fn new(state: ImageHistState) -> Self {
        Self {
            id: egui::Id::new(uuid::Uuid::new_v4()),
            state,
        }
    }
}

pub struct TabViewer {
    tabs_count: usize,
    filepath_tx: mpsc::Sender<(NodePath, String, Vec<u8>)>,
}

impl TabViewer {
    pub fn new(tabs_count: usize, filepath_tx: mpsc::Sender<(NodePath, String, Vec<u8>)>) -> Self {
        Self {
            tabs_count,
            filepath_tx,
        }
    }
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = ImageHistTab;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        tab.id
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        let tab_image_path = tab.state.image_path();
        if let Some((_, suffix)) = tab_image_path.rsplit_once('/')
            && !suffix.is_empty()
            && tab_image_path.starts_with('/')
        {
            suffix
        } else {
            "???"
        }
        .into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let mut image_hist = ImageHist::new(tab.id, &mut tab.state);
        ui.add(&mut image_hist);
    }

    fn on_add(&mut self, path: NodePath) {
        let task = rfd::AsyncFileDialog::new()
            .add_filter("image", &["png", "jpg", "jpeg", "svg"])
            .set_directory("/")
            .pick_file();

        let tx = self.filepath_tx.clone();

        let pick_file_task = async move {
            let file = task.await;
            if let Some(file_handle) = file {
                let data = file_handle.read().await;
                let filepath = file_handle.path().to_string_lossy();
                if let Err(err) = tx.send((path, filepath.to_string(), data)) {
                    warn!("Ошибка при записи в канал (файл {filepath}): {err}");
                }
            }
        };
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(pick_file_task);
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || pollster::block_on(pick_file_task));
    }

    fn is_closeable(&self, _tab: &Self::Tab) -> bool {
        self.tabs_count > 1
    }

    // fn on_rect_changed(&mut self, _tab: &mut Self::Tab) {}
}
