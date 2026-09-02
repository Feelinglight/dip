use egui::TextureHandle;
use image::GrayImage;
use intensity::histogram::{Histogram, histogram};

use super::image_viewport::ImageViewportState;
use crate::{pipeline::Pipeline, widgets::transforms_panel::TransformEditorCache};

/// Сохраняется между перезапусками приложения и относится только к бизнес логике приложения
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub(super) struct ImageHistConfig {
    pub(super) image_path: String,
    pub(super) pipeline: Pipeline,
}

pub(super) struct LoadedImage {
    original: GrayImage,
    transformed: GrayImage,
    histogram: Histogram,
}

impl LoadedImage {
    pub(super) fn from_original(original: GrayImage, pipeline: &Pipeline) -> LoadedImage {
        let (transformed, histogram) = Self::transform_original(&original, pipeline);
        Self {
            original,
            transformed,
            histogram,
        }
    }

    pub(super) fn retransform(&mut self, pipeline: &Pipeline) {
        (self.transformed, self.histogram) = Self::transform_original(&self.original, pipeline);
    }

    pub(super) fn transformed(&self) -> &GrayImage {
        &self.transformed
    }

    pub(super) fn histogram(&self) -> &Histogram {
        &self.histogram
    }

    fn transform_original(original: &GrayImage, pipeline: &Pipeline) -> (GrayImage, Histogram) {
        let mut transformed = original.clone();
        pipeline.apply_to(&mut transformed);
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
    pub(super) texture: Option<TextureHandle>,
    pub(super) transform_editor_cache: TransformEditorCache,
}

impl Default for ImageHistRuntimeState {
    fn default() -> Self {
        Self {
            image: ImageLoadState::Empty,
            texture: None,
            transform_editor_cache: TransformEditorCache::default(),
        }
    }
}

/// Сохраняется между перезапусками приложения и относится только к UI
#[derive(serde::Deserialize, serde::Serialize)]
pub(super) struct ImageHistViewState {
    pub(super) viewport: ImageViewportState,
    pub(super) hist_enable: bool,

    pub(super) show_image_controls: bool,

    pub(super) transforms_viewport_pos: Option<egui::Pos2>,
    pub(super) transforms_viewport_size: Option<egui::Vec2>,
}

impl Default for ImageHistViewState {
    fn default() -> Self {
        Self {
            viewport: ImageViewportState::default(),
            hist_enable: true,
            show_image_controls: false,
            transforms_viewport_size: None,
            transforms_viewport_pos: None,
        }
    }
}
