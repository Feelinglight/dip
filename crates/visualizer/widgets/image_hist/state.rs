use std::path::Path;

use egui::{ColorImage, TextureHandle};
use image::GrayImage;
use intensity::hist::{HistArray, empty_hist, make_hist};

use crate::widgets::transforms_panel::AppliedTransform;
use crate::widgets::zoom_texture::ZoomTextureState;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ImageHistState {
    pub zt_state: ZoomTextureState,
    pub transforms: Vec<AppliedTransform>,
    pub image_path_edit_text: String,
    pub hist_enable: bool,

    pub transforms_viewport_size: Option<egui::Vec2>,
    pub transforms_viewport_pos: Option<egui::Pos2>,

    #[serde(skip)]
    pub run: ImageHistRunState,
}

pub struct Images {
    pub original: GrayImage,
    pub transformed: GrayImage,
}

pub struct ImageHistRunState {
    pub images: Option<Images>,
    pub show_image_controls: bool,
    pub hist: HistArray,
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
            hist: empty_hist(),
        }
    }
}

fn load_gray_image(path: &Path, image_data: Option<&[u8]>) -> Result<GrayImage, String> {
    let image_path = path.to_str().ok_or(
        "Ошибка. Не удалось загрузить изображение: путь содержит не UTF-8 символы".to_string(),
    )?;

    let img = if let Some(data) = image_data {
        image::load_from_memory(data)
    } else {
        image::open(image_path)
    }
    .map_err(|e| e.to_string())?;

    Ok(img.to_luma8())
}

fn load_egui_texture(ctx: &egui::Context, image: &GrayImage) -> TextureHandle {
    let colored_image = ColorImage::from_gray(
        [image.width() as usize, image.height() as usize],
        image.as_flat_samples().as_slice(),
    );
    ctx.load_texture("dip", colored_image, egui::TextureOptions::LINEAR)
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
        state.update_hist();

        Ok(state)
    }

    pub fn from_path(ctx: &egui::Context, path: &Path) -> Result<Self, &'static str> {
        if let Some(image_path) = path.to_str() {
            let mut state = Self {
                image_path_edit_text: String::from(image_path),
                ..Default::default()
            };
            state.reload_image(ctx);
            Ok(state)
        } else {
            Err("Ошибка. Не удалось загрузить изображение: путь содержит не UTF-8 символы")
        }
    }

    /// Загружает изображение по текущему установленному пути и обновляет его гистограмму
    pub fn reload_image(&mut self, ctx: &egui::Context) {
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

    pub fn image_path(&self) -> &String {
        &self.image_path_edit_text
    }

    pub fn restore(&mut self, ctx: &egui::Context) {
        self.reload_image(ctx);
        for transform in &mut self.transforms {
            transform.op.restore_state();
        }
    }

    fn set_images(&mut self, ctx: &egui::Context, original_image: GrayImage) {
        let mut transformed = original_image.clone();

        Self::apply_active_transforms(&self.transforms, &mut transformed);
        self.update_hist();
        self.zt_state
            .set_texture(load_egui_texture(ctx, &transformed), false);

        self.run.images = Some(Images {
            original: original_image,
            transformed,
        });
    }

    fn apply_active_transforms(transforms: &[AppliedTransform], image: &mut GrayImage) {
        for transform in transforms {
            transform.apply_if_active(image);
        }
    }

    pub fn apply_transforms(&mut self, ctx: &egui::Context) {
        if let Some(images) = &mut self.run.images {
            let mut new_transformed = images.original.clone();

            Self::apply_active_transforms(&self.transforms, &mut new_transformed);
            self.zt_state
                .set_texture(load_egui_texture(ctx, &new_transformed), false);

            images.transformed = new_transformed;
        }
        self.update_hist();
    }

    /// Сбрасывает текущее загруженное изображение в его первоначальное состояние
    /// Если изображение не загружено, то не делает ничего
    pub fn reset_zoom_texture(&mut self) {
        self.zt_state.reset_parameters();
    }

    /// Строит гистограмму для текущего изображения
    fn update_hist(&mut self) {
        self.run.hist = if let Some(Images { transformed, .. }) = &self.run.images {
            make_hist(transformed)
        } else {
            empty_hist()
        }
    }
}
