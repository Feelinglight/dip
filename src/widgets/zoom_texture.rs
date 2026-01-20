use std::path::{Path, PathBuf};

use crate::errors::LoadImageError;
use egui::{
    Align2, Color32, ColorImage, Context, FontId, Painter, Pos2, Rect, TextureHandle, Vec2, Widget,
};
use image::ImageReader;

const MIN_ZOOM: f32 = 0.05;
const MAX_ZOOM: f32 = 20.;
const ZOOM_SPEED: f32 = 0.005;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ZoomTextureState {
    image_path: Option<PathBuf>,

    #[serde(skip)]
    texture: Option<Result<TextureHandle, LoadImageError>>,

    zoom: f32,
    pan: Vec2,
}

impl Default for ZoomTextureState {
    fn default() -> Self {
        Self {
            image_path: None,
            texture: None,
            zoom: 1.,
            pan: Vec2::ZERO,
        }
    }
}

impl ZoomTextureState {
    /// Перезагружает картинку по уже установленному пути `self.image_path`
    pub fn reload_image(&mut self, ctx: &Context) {
        if let Some(path) = &self.image_path {
            self.texture = Some(self._load_image(ctx, path));
        }
    }

    /// Загружает картинку по заданному пути
    pub fn load_image(&mut self, ctx: &Context, path: &Path) {
        *self = Self {
            image_path: Some(path.to_path_buf()),
            texture: Some(self._load_image(ctx, path)),
            ..Default::default()
        }
    }

    fn _load_image(&self, ctx: &Context, path: &Path) -> Result<TextureHandle, LoadImageError> {
        let rgb_image = ImageReader::open(path)?
            .with_guessed_format()?
            .decode()?
            .to_rgba8();

        let colored_image = ColorImage::from_rgba_unmultiplied(
            [rgb_image.width() as usize, rgb_image.height() as usize],
            rgb_image.as_flat_samples().as_slice(),
        );

        Ok(ctx.load_texture("dip", colored_image, egui::TextureOptions::LINEAR))
    }

    /// Сбрасывает параметры отображения картинки, такие как zoom и pan
    pub fn reset_parameters(&mut self) {
        self.zoom = 1.;
        self.pan = Vec2::ZERO;
    }
}

pub struct ZoomTexture<'a> {
    state: &'a mut ZoomTextureState,
    available_size: Vec2,
}

impl<'a> ZoomTexture<'a> {
    pub fn new(state: &'a mut ZoomTextureState, available_size: Vec2) -> ZoomTexture<'a> {
        Self {
            state,
            available_size,
        }
    }

    /// Рисует на painter белый прямоугольник с текстом ошибки
    fn show_error(&self, painter: &Painter, text: &str, rect_size: Vec2) {
        painter.rect_filled(
            Rect {
                min: Pos2 { x: 0., y: 0. },
                max: Pos2 {
                    x: rect_size.x,
                    y: rect_size.y,
                },
            },
            0.,
            Color32::WHITE,
        );
        painter.text(
            Pos2 {
                x: rect_size.x / 2.,
                y: rect_size.y / 2.,
            },
            Align2::CENTER_CENTER,
            text,
            FontId::default(),
            Color32::BLACK,
        );
    }
}

impl<'a> Widget for ZoomTexture<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) = ui.allocate_painter(self.available_size, egui::Sense::drag());

        match &self.state.texture {
            None => {
                self.show_error(&painter, "Изображение\nне найдено", self.available_size);
            }
            Some(Err(err)) => {
                self.show_error(&painter, &format!("{}", err), self.available_size);
            }
            Some(Ok(texture)) => {
                // Подгон зума так, чтобы картинка занимала все доступное пространство при первом
                // отображении и при этом не выходила за его границы
                let image_size = texture.size_vec2();
                let fitted_zoom = fit_zoom(self.available_size, image_size);
                let zoom = fitted_zoom * self.state.zoom;

                // Обработка зума колесиком
                if response.hovered() {
                    let scroll = ui.input(|i| i.raw_scroll_delta.y);
                    if scroll != 0.0 {
                        let zoom_factor = (scroll * ZOOM_SPEED).exp();
                        self.state.zoom = (self.state.zoom * zoom_factor).clamp(MIN_ZOOM, MAX_ZOOM);
                    }
                }

                // Перетаскивание мышью
                if response.dragged() {
                    self.state.pan += response.drag_delta();
                }

                let zoomed_size = image_size * zoom;
                let rect = egui::Rect::from_min_size(
                    response.rect.center() - zoomed_size / 2.0 + self.state.pan,
                    zoomed_size,
                );

                painter.image(
                    texture.id(),
                    rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
        }
        response
    }
}

// /// Рассчитывает зум, при котором картинка с размером image_size будет занимать ровно
// /// available_size
fn fit_zoom(available_space: Vec2, image_size: Vec2) -> f32 {
    (available_space.x / image_size.x)
        .min(available_space.y / image_size.y)
        .max(0.001)
}
