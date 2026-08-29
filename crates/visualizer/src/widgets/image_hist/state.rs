use std::path::Path;

use image::GrayImage;
use intensity::histogram::{HistArray, empty_histogram, histogram};

use crate::widgets::transforms_panel::AppliedTransform;
use crate::widgets::zoom_texture::ZoomTextureState;

use super::load_image::{load_gray_image, load_texture};

pub(super) struct Images {
    pub original: GrayImage,
    pub transformed: GrayImage,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ImageHistState {
    zt_state: ZoomTextureState,
    transforms: Vec<AppliedTransform>,
    image_path_edit_text: String,
    hist_enable: bool,

    transforms_viewport_size: Option<egui::Vec2>,
    transforms_viewport_pos: Option<egui::Pos2>,

    #[serde(skip)]
    run: ImageHistRunState,
}

struct ImageHistRunState {
    images: Option<Images>,
    show_image_controls: bool,
    hist: HistArray,
}

impl Default for ImageHistState {
    fn default() -> Self {
        Self {
            zt_state: ZoomTextureState::default(),
            transforms: Vec::default(),
            image_path_edit_text: String::new(),
            hist_enable: true,
            transforms_viewport_size: None,
            transforms_viewport_pos: None,
            run: ImageHistRunState::default(),
        }
    }
}

impl Default for ImageHistRunState {
    fn default() -> Self {
        Self {
            images: None,
            show_image_controls: false,
            hist: empty_histogram(),
        }
    }
}

impl ImageHistState {
    /// Загружает изображение из массива данных ``data``. Устанавливает путь к изображению в
    /// ``path``.
    /// Путь к изображению требуется для повторной загрузки изображения с помощью метода
    /// ``reload_image``
    pub fn from_memory(ctx: &egui::Context, path: &Path, data: &[u8]) -> Result<Self, String> {
        let gray_image = load_gray_image(path, Some(data))
            .map_err(|e| format!("Ошибка. Не удалось загрузить изображение: {e}"))?;

        let mut state = Self {
            image_path_edit_text: path
                .to_str()
                .expect("Путь к изображению не валиден")
                .to_string(),
            ..Default::default()
        };

        state.set_images(ctx, gray_image);

        Ok(state)
    }

    /// Загружает изображение по текущему установленному пути и обновляет его гистограмму
    pub(super) fn reload_image(&mut self, ctx: &egui::Context) {
        let image = load_gray_image(Path::new(&self.image_path_edit_text), None);

        match image {
            Ok(img) => {
                self.set_images(ctx, img);
            }
            Err(message) => {
                self.run.images = None;
                self.zt_state.set_error(message);
            }
        }

        self.update_hist();
    }

    pub fn image_path(&self) -> &str {
        &self.image_path_edit_text
    }

    pub fn restore(&mut self, ctx: &egui::Context) {
        self.reload_image(ctx);
        for transform in &mut self.transforms {
            transform.op.restore_state();
        }
    }

    fn set_images(&mut self, ctx: &egui::Context, original_image: image::GrayImage) {
        let mut transformed = original_image.clone();
        Self::apply_active_transforms(&self.transforms, &mut transformed);

        self.zt_state
            .set_texture(load_texture(ctx, &transformed), false);

        self.run.images = Some(Images {
            original: original_image,
            transformed,
        });
        self.update_hist();
    }

    fn apply_active_transforms(transforms: &[AppliedTransform], image: &mut image::GrayImage) {
        for transform in transforms {
            transform.apply_if_active(image);
        }
    }

    pub(super) fn apply_transforms(&mut self, ctx: &egui::Context) {
        if let Some(images) = &mut self.run.images {
            let mut new_transformed = images.original.clone();

            Self::apply_active_transforms(&self.transforms, &mut new_transformed);
            self.zt_state
                .set_texture(load_texture(ctx, &new_transformed), false);

            images.transformed = new_transformed;
        }
        self.update_hist();
    }

    /// Строит гистограмму для текущего изображения
    fn update_hist(&mut self) {
        self.run.hist = if let Some(Images { transformed, .. }) = &self.run.images {
            histogram(transformed)
        } else {
            empty_histogram()
        }
    }
    /// Сбрасывает текущее загруженное изображение в его первоначальное состояние
    /// Если изображение не загружено, то не делает ничего
    pub(super) fn reset_zoom_texture(&mut self) {
        self.zt_state.reset_parameters();
    }

    pub(super) fn zoom_texture_state_mut(&mut self) -> &mut ZoomTextureState {
        &mut self.zt_state
    }

    pub(super) fn image_path_mut(&mut self) -> &mut String {
        &mut self.image_path_edit_text
    }

    pub(super) fn histogram(&self) -> &HistArray {
        &self.run.hist
    }

    pub(super) fn histogram_enabled(&self) -> bool {
        self.hist_enable
    }

    pub(super) fn toggle_histogram(&mut self) {
        self.hist_enable = !self.hist_enable;
    }

    pub(super) fn open_transforms_viewport(&mut self) {
        self.run.show_image_controls = true;
    }

    pub(super) fn close_transforms_viewport(&mut self) {
        self.run.show_image_controls = false;
    }

    pub(super) fn transforms_viewport_open(&self) -> bool {
        self.run.show_image_controls
    }

    pub(super) fn transforms_viewport_size(&self) -> Option<egui::Vec2> {
        self.transforms_viewport_size
    }

    pub(super) fn set_transforms_viewport_size(&mut self, size: egui::Vec2) {
        self.transforms_viewport_size = Some(size);
    }

    pub(super) fn transforms_viewport_pos(&self) -> Option<egui::Pos2> {
        self.transforms_viewport_pos
    }

    pub(super) fn set_transforms_viewport_pos(&mut self, pos: egui::Pos2) {
        self.transforms_viewport_pos = Some(pos);
    }

    pub(super) fn transforms_mut(&mut self) -> &mut Vec<AppliedTransform> {
        &mut self.transforms
    }
}
