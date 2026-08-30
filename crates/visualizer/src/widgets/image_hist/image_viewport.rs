use egui::{Align2, Color32, FontId, Painter, Pos2, Rect, TextureHandle, Vec2, Widget};

const MIN_ZOOM: f32 = 0.05;
const MAX_ZOOM: f32 = 20.;
const ZOOM_SPEED: f32 = 0.005;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ImageViewportState {
    zoom: f32,
    pan: Vec2,
}

impl Default for ImageViewportState {
    fn default() -> Self {
        Self {
            zoom: 1.,
            pan: Vec2::ZERO,
        }
    }
}

impl ImageViewportState {
    pub fn reset(&mut self) {
        *self = ImageViewportState::default();
    }
}

pub enum ImageViewportContent<'a> {
    Texture(&'a TextureHandle),
    Message(&'a str),
}

pub struct ImageViewport<'state, 'content> {
    state: &'state mut ImageViewportState,
    texture: ImageViewportContent<'content>,
    available_size: Vec2,
}

impl<'state, 'content> ImageViewport<'state, 'content> {
    pub fn new(
        state: &'state mut ImageViewportState,
        texture: ImageViewportContent<'content>,
        available_size: Vec2,
    ) -> ImageViewport<'state, 'content> {
        Self {
            state,
            texture,
            available_size,
        }
    }

    // Рисует на painter белый прямоугольник с текстом ошибки
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

    /// Рассчитывает зум, при котором картинка с размером `image_size` будет занимать ровно
    /// `available_size`
    fn fit_zoom(available_space: Vec2, image_size: Vec2) -> f32 {
        (available_space.x / image_size.x)
            .min(available_space.y / image_size.y)
            .max(0.001)
    }
}

impl Widget for ImageViewport<'_, '_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(self.available_size, egui::Sense::click_and_drag());

        match self.texture {
            ImageViewportContent::Message(message) => {
                ImageViewport::show_error(&painter, message, self.available_size);
            }
            ImageViewportContent::Texture(texture) => {
                // Подгон зума так, чтобы картинка занимала все доступное пространство при первом
                // отображении и при этом не выходила за его границы
                let image_size = texture.size_vec2();
                let fitted_zoom = ImageViewport::fit_zoom(self.available_size, image_size);
                let zoom = fitted_zoom * self.state.zoom;

                // Обработка зума колесиком
                if response.hovered() {
                    let scroll = ui.input(|i| i.smooth_scroll_delta.y);
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
