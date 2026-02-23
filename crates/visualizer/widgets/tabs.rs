use std::sync::mpsc;

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
    filepath_tx: mpsc::Sender<(
        egui_dock::SurfaceIndex,
        egui_dock::NodeIndex,
        String,
        Vec<u8>,
    )>,
}

impl TabViewer {
    pub fn new(
        filepath_tx: mpsc::Sender<(
            egui_dock::SurfaceIndex,
            egui_dock::NodeIndex,
            String,
            Vec<u8>,
        )>,
    ) -> Self {
        Self { filepath_tx }
    }
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = ImageHistTab;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        tab.id
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.state.image_path().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let mut image_hist = ImageHist::new(tab.id, &mut tab.state);
        ui.add(&mut image_hist);
    }

    fn on_add(&mut self, surface: egui_dock::SurfaceIndex, node: egui_dock::NodeIndex) {
        let task = rfd::AsyncFileDialog::new()
            .add_filter("image", &["png", "jpg", "jpeg", "svg"])
            .set_directory("/")
            .pick_file();

        let tx = self.filepath_tx.clone();

        let pick_file_task = async move {
            let file = task.await;
            if let Some(file_handle) = file {
                let data = file_handle.read().await;
                if let Err(err) = tx.send((surface, node, file_handle.file_name(), data)) {
                    warn!(
                        "Ошибка при записи в канал (файл {}): {}",
                        file_handle.file_name(),
                        err
                    );
                }
            }
        };
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(pick_file_task);
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || pollster::block_on(pick_file_task));
    }

    fn on_close(&mut self, _tab: &mut Self::Tab) -> egui_dock::tab_viewer::OnCloseResponse {
        egui_dock::tab_viewer::OnCloseResponse::Ignore
    }

    // fn on_rect_changed(&mut self, _tab: &mut Self::Tab) {}
}
