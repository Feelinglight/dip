use image::GrayImage;
use intensity;

use crate::widgets::{transforms_panel::AppliedTransform, zoom_texture::ZoomTextureState};

/// Сохраняется между перезапусками приложения и относится только к бизнес логике приложения
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub(super) struct ImageHistConfig {
    pub(super) image_path: String,
    pub(super) transforms: Vec<AppliedTransform>,
}

pub(super) struct LoadedImage {
    pub(super) original: GrayImage,
    pub(super) transformed: GrayImage,
    pub(super) histogram: Box<intensity::histogram::Histogram>,
}

impl LoadedImage {
    pub(super) fn from_original(
        original: GrayImage,
        transforms: &[AppliedTransform],
    ) -> LoadedImage {
        let mut transformed = original.clone();
        for transform in transforms {
            transform.apply_if_active(&mut transformed);
        }
        Self {
            original,
            histogram: Box::new(intensity::histogram::histogram(&transformed)),
            transformed,
        }
    }

    pub(super) fn retransform(&mut self, transforms: &[AppliedTransform]) {
        self.transformed = self.original.clone();

        for transform in transforms {
            transform.apply_if_active(&mut self.transformed);
        }

        *self.histogram = intensity::histogram::histogram(&self.transformed);
    }
}

/// Не сохраняется между запусками приложения и инициализируется при запуске из сохраняемых данных
pub(super) struct ImgaeHistRuntimeState {
    pub(super) image: Result<LoadedImage, String>,
}

impl Default for ImgaeHistRuntimeState {
    fn default() -> Self {
        Self {
            image: Result::Err("Изображение не загружено".to_string()),
        }
    }
}

/// Сохраняется между перезапусками приложения и относится только к UI
#[derive(serde::Deserialize, serde::Serialize)]
pub(super) struct ImageHistViewState {
    pub(super) zt_state: ZoomTextureState,
    pub(super) image_path_edit_text: String,
    pub(super) hist_enable: bool,

    pub(super) show_image_controls: bool,

    pub(super) transforms_viewport_pos: Option<egui::Pos2>,
    pub(super) transforms_viewport_size: Option<egui::Vec2>,
}

impl Default for ImageHistViewState {
    fn default() -> Self {
        Self {
            zt_state: ZoomTextureState::default(),
            image_path_edit_text: String::new(),
            hist_enable: true,
            show_image_controls: false,
            transforms_viewport_size: None,
            transforms_viewport_pos: None,
        }
    }
}
