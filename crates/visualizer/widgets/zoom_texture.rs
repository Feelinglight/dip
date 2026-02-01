use egui::{
    Align2, Color32, ColorImage, Context, FontId, Painter, Pos2, Rect, TextureHandle, Vec2, Widget,
};
use image::GrayImage;

use crate::errors::LoadImageError;

const MIN_ZOOM: f32 = 0.05;
const MAX_ZOOM: f32 = 20.;
const ZOOM_SPEED: f32 = 0.005;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ZoomTextureState {
    #[serde(skip)]
    texture: Result<TextureHandle, String>,

    zoom: f32,
    pan: Vec2,
}

impl Default for ZoomTextureState {
    fn default() -> Self {
        Self {
            texture: Err(String::from("Выберите изображение")),
            zoom: 1.,
            pan: Vec2::ZERO,
        }
    }
}

impl ZoomTextureState {
    /// Загружает картинку по заданному пути
    /// Если установлен флаг ``reset_params``, то сбрасывает параметры отображения картинки,
    /// такие как zoom и pan
    pub fn set_texture(
        &mut self,
        ctx: &Context,
        image: Result<&GrayImage, &LoadImageError>,
        reset_params: bool,
    ) {
        self.texture = match image {
            Ok(img) => {
                let colored_image = ColorImage::from_gray(
                    [img.width() as usize, img.height() as usize],
                    img.as_flat_samples().as_slice(),
                );
                Ok(ctx.load_texture("dip", colored_image, egui::TextureOptions::LINEAR))
            }
            Err(load_error) => Err(load_error.to_string()),
        };

        if reset_params {
            self.reset_parameters();
        }
    }

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
    fn show_error(painter: &Painter, text: &str, rect_size: Vec2) {
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

impl Widget for ZoomTexture<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(self.available_size, egui::Sense::click_and_drag());

        match &self.state.texture {
            Err(err) => {
                ZoomTexture::show_error(&painter, err, self.available_size);
            }
            Ok(texture) => {
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
