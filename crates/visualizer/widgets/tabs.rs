use std::sync::mpsc;

use egui_dock::NodePath;
use log::warn;
use rfd::FileHandle;

use crate::widgets::image_hist::{ImageHist, ImageHistState};

pub struct OpenedImage {
    pub node_path: NodePath,
    pub path: String,
    pub data: Vec<u8>,
}

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

pub struct TabViewer<'a> {
    tabs_count: usize,
    default_folder: &'a String,
    filepath_tx: mpsc::Sender<OpenedImage>,
}

impl<'a> TabViewer<'a> {
    pub fn new(
        tabs_count: usize,
        default_folder: &'a String,
        filepath_tx: mpsc::Sender<OpenedImage>,
    ) -> Self {
        Self {
            tabs_count,
            default_folder,
            filepath_tx,
        }
    }
}

impl egui_dock::TabViewer for TabViewer<'_> {
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
            tab_image_path
        }
        .into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let mut image_hist = ImageHist::new(tab.id, &mut tab.state);
        ui.add(&mut image_hist);

        if image_hist.open_image_requested() {
            self.add_image_tab(NodePath::right_node(NodePath::MAIN_ROOT));
        }
    }

    fn on_add(&mut self, path: NodePath) {
        self.add_image_tab(path);
    }

    fn is_closeable(&self, _tab: &Self::Tab) -> bool {
        self.tabs_count > 1
    }
}

impl TabViewer<'_> {
    fn add_image_tab(&mut self, node_path: NodePath) {
        let task = rfd::AsyncFileDialog::new()
            .add_filter("image", &["png", "jpg", "jpeg", "svg"])
            .set_directory(self.default_folder)
            .pick_file();

        let tx = self.filepath_tx.clone();

        let pick_file_task = async move {
            let file = task.await;
            if let Some(file_handle) = file {
                let data = file_handle.read().await;
                let filename = TabViewer::filename(&file_handle);
                if let Err(err) = tx.send(OpenedImage {
                    node_path,
                    path: filename.clone(),
                    data,
                }) {
                    warn!("Ошибка при записи в канал (файл {filename}): {err}");
                }
            }
        };
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(pick_file_task);
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || pollster::block_on(pick_file_task));
    }

    // В wasm у FileHandle нет метода path.
    // Но этот метод полезен для повторной загрузки изображения (reload). Если сохранять только
    // имя, то повторная загрузка будет работать только по относительному пути.
    fn filename(file_handle: &FileHandle) -> String {
        #[cfg(target_arch = "wasm32")]
        return file_handle.file_name();
        #[cfg(not(target_arch = "wasm32"))]
        return file_handle.path().to_string_lossy().to_string();
    }
}
