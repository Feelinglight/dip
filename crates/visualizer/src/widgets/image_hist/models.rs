use image::GrayImage;
use intensity::histogram::{Histogram, histogram};

use crate::widgets::{transforms_panel::AppliedTransform, zoom_texture::ZoomTextureState};

/// Сохраняется между перезапусками приложения и относится только к бизнес логике приложения
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub(super) struct ImageHistConfig {
    pub(super) image_path: String,
    pub(super) transforms: Vec<AppliedTransform>,
}

pub(super) struct LoadedImage {
    original: GrayImage,
    transformed: GrayImage,
    histogram: Histogram,
}

impl LoadedImage {
    pub(super) fn from_original(
        original: GrayImage,
        transforms: &[AppliedTransform],
    ) -> LoadedImage {
        let (transformed, histogram) = Self::transform_original(&original, transforms);
        Self {
            original,
            transformed,
            histogram,
        }
    }

    pub(super) fn retransform(&mut self, transforms: &[AppliedTransform]) {
        (self.transformed, self.histogram) = Self::transform_original(&self.original, transforms);
    }

    pub(super) fn transformed(&self) -> &GrayImage {
        &self.transformed
    }

    pub(super) fn histogram(&self) -> &Histogram {
        &self.histogram
    }

    fn transform_original(
        original: &GrayImage,
        transforms: &[AppliedTransform],
    ) -> (GrayImage, Histogram) {
        let mut transformed = original.clone();
        for transform in transforms {
            transform.apply_if_active(&mut transformed);
        }
        let histogram = histogram(&transformed);
        (transformed, histogram)
    }
}

pub(super) enum ImageLoadState {
    Empty,
    Loaded(Box<LoadedImage>),
    Failed(String),
}

/// Не сохраняется между запусками приложения и инициализируется при запуске из сохраняемых данных
pub(super) struct ImageHistRuntimeState {
    pub(super) image: ImageLoadState,
}

impl Default for ImageHistRuntimeState {
    fn default() -> Self {
        Self {
            image: ImageLoadState::Empty,
        }
    }
}

/// Сохраняется между перезапусками приложения и относится только к UI
#[derive(serde::Deserialize, serde::Serialize)]
pub(super) struct ImageHistViewState {
    pub(super) zt_state: ZoomTextureState,
    pub(super) hist_enable: bool,

    pub(super) show_image_controls: bool,

    pub(super) transforms_viewport_pos: Option<egui::Pos2>,
    pub(super) transforms_viewport_size: Option<egui::Vec2>,
}

impl Default for ImageHistViewState {
    fn default() -> Self {
        Self {
            zt_state: ZoomTextureState::default(),
            hist_enable: true,
            show_image_controls: false,
            transforms_viewport_size: None,
            transforms_viewport_pos: None,
        }
    }
}
