use std::sync::mpsc;

use log::warn;
use rfd::FileHandle;

pub struct PickedImage<T> {
    pub path: String,
    pub bytes: Vec<u8>,
    pub user_data: T,
}

#[derive(Debug)]
pub struct ImagePicker<T> {
    pick_path: String,

    picked_image_tx: mpsc::Sender<PickedImage<T>>,
    picked_image_rx: mpsc::Receiver<PickedImage<T>>,
}

impl<T: std::marker::Send + 'static> ImagePicker<T> {
    /// - `pick_path` - путь в файловой системе, где будет открыт файловый менеджер.
    ///   Может быть путем к папке или файлу. Если это путь к файлу, то будет открыта родительская
    ///   папка
    pub fn new(pick_path: &str) -> Self {
        let (picked_image_tx, picked_image_rx) = mpsc::channel();
        Self {
            pick_path: pick_path.to_string(),
            picked_image_tx,
            picked_image_rx,
        }
    }

    pub fn get_pick_path(&self) -> &str {
        &self.pick_path
    }

    pub fn request_image(&self, user_data: T) {
        self.open_pick_window(user_data);
    }

    pub fn poll_picked_image(&mut self, on_pick: impl FnOnce(PickedImage<T>)) {
        match self.picked_image_rx.try_recv() {
            Ok(picked_image) => {
                self.pick_path.clone_from(&picked_image.path);
                on_pick(picked_image);
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                warn!("Канал отключился до того как изображение было принято");
            }
            Err(mpsc::TryRecvError::Empty) => {}
        }
    }

    fn open_pick_window(&self, user_data: T) {
        let task = rfd::AsyncFileDialog::new()
            .add_filter("image", &["png", "jpg", "jpeg", "svg"])
            // Если pick_path - путь к файлу, то будет открыта родительская папка
            .set_directory(self.pick_path.clone())
            .pick_file();

        let image_tx = self.picked_image_tx.clone();

        let pick_file_task = async move {
            let file = task.await;
            if let Some(file_handle) = file {
                let _ = image_tx.send(PickedImage {
                    user_data,
                    path: ImagePicker::<T>::filename(&file_handle),
                    bytes: file_handle.read().await,
                });
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
