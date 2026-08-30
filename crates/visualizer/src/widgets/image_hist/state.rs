use super::models::{ImageHistConfig, ImageHistViewState, ImgaeHistRuntimeState};
use std::path::Path;

use intensity::histogram::Histogram;

use crate::widgets::zoom_texture::ZoomTextureState;
use crate::widgets::{image_hist::models::LoadedImage, transforms_panel::AppliedTransform};

use super::load_image::{load_gray_image, load_texture};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct ImageHistState {
    pub(super) config: ImageHistConfig,
    pub(super) view: ImageHistViewState,

    #[serde[skip]]
    pub(super) runtime: ImgaeHistRuntimeState,
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
            config: ImageHistConfig {
                image_path: path
                    .to_str()
                    .expect("Путь к изображению не валиден")
                    .to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        state.set_images(ctx, gray_image);

        Ok(state)
    }

    /// Загружает изображение по текущему установленному пути и обновляет его гистограмму
    pub(super) fn reload_image(&mut self, ctx: &egui::Context) {
        let image = load_gray_image(Path::new(&self.config.image_path), None);

        match image {
            Ok(img) => {
                self.set_images(ctx, img);
            }
            Err(message) => {
                self.runtime.image = Err(message.clone());
                self.view.zt_state.set_error(message);
            }
        }
    }

    pub fn image_path(&self) -> &str {
        &self.config.image_path
    }

    pub fn restore(&mut self, ctx: &egui::Context) {
        self.reload_image(ctx);
        for transform in &mut self.config.transforms {
            transform.op.restore_state();
        }
    }

    fn set_images(&mut self, ctx: &egui::Context, original_image: image::GrayImage) {
        let image = LoadedImage::from_original(original_image, &self.config.transforms);

        self.view
            .zt_state
            .set_texture(load_texture(ctx, &image.transformed), false);

        self.runtime.image = Ok(image);
    }

    pub(super) fn apply_transforms(&mut self, ctx: &egui::Context) {
        if let Ok(image) = &mut self.runtime.image {
            image.retransform(&self.config.transforms);

            self.view
                .zt_state
                .set_texture(load_texture(ctx, &image.transformed), false);
        }
    }

    /// Сбрасывает текущее загруженное изображение в его первоначальное состояние
    /// Если изображение не загружено, то не делает ничего
    pub(super) fn reset_zoom_texture(&mut self) {
        self.view.zt_state.reset_parameters();
    }

    pub(super) fn zoom_texture_state_mut(&mut self) -> &mut ZoomTextureState {
        &mut self.view.zt_state
    }

    pub(super) fn image_path_mut(&mut self) -> &mut String {
        &mut self.config.image_path
    }

    pub(super) fn histogram(&self) -> Option<&Histogram> {
        if let Ok(image) = &self.runtime.image {
            Some(&image.histogram)
        } else {
            None
        }
    }

    pub(super) fn histogram_enabled(&self) -> bool {
        self.view.hist_enable
    }

    pub(super) fn toggle_histogram(&mut self) {
        self.view.hist_enable = !self.view.hist_enable;
    }

    pub(super) fn open_transforms_viewport(&mut self) {
        self.view.show_image_controls = true;
    }

    pub(super) fn close_transforms_viewport(&mut self) {
        self.view.show_image_controls = false;
    }

    pub(super) fn transforms_viewport_open(&self) -> bool {
        self.view.show_image_controls
    }

    pub(super) fn transforms_viewport_size(&self) -> Option<egui::Vec2> {
        self.view.transforms_viewport_size
    }

    pub(super) fn set_transforms_viewport_size(&mut self, size: egui::Vec2) {
        self.view.transforms_viewport_size = Some(size);
    }

    pub(super) fn transforms_viewport_pos(&self) -> Option<egui::Pos2> {
        self.view.transforms_viewport_pos
    }

    pub(super) fn set_transforms_viewport_pos(&mut self, pos: egui::Pos2) {
        self.view.transforms_viewport_pos = Some(pos);
    }

    pub(super) fn transforms_mut(&mut self) -> &mut Vec<AppliedTransform> {
        &mut self.config.transforms
    }
}
