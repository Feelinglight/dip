use egui::{TextureHandle, Vec2, Widget};

const MIN_ZOOM: f32 = 0.05;
const MAX_ZOOM: f32 = 20.;
const ZOOM_SPEED: f32 = 0.005;

pub struct ZoomWidgetConfig {
    texture: TextureHandle,
    image_size: Vec2,

    zoom: f32,
    pan: Vec2,
}

pub struct ZoomWidget<'a> {
    config: &'a mut ZoomWidgetConfig,
    available_size: Vec2,
}

impl ZoomWidgetConfig {
    pub fn new(texture: TextureHandle) -> ZoomWidgetConfig {
        let size = texture.size_vec2();
        Self {
            texture: texture,
            image_size: size,
            zoom: 1.,
            pan: Vec2::ZERO,
        }
    }
}

impl<'a> ZoomWidget<'a> {
    pub fn new(config: &'a mut ZoomWidgetConfig, available_size: Vec2) -> ZoomWidget<'a> {
        Self {
            config: config,
            available_size: available_size,
        }
    }
}

impl<'a> Widget for ZoomWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) = ui.allocate_painter(self.available_size, egui::Sense::drag());

        // Подгон зума так, чтобы картинка занимала все доступное пространство при первом
        // отображении и при этом не выходила за его границы
        let fitted_zoom = fit_zoom(self.available_size, self.config.image_size);
        let zoom = fitted_zoom * self.config.zoom;

        // Обработка зума колесиком
        if response.hovered() {
            let scroll = ui.input(|i| i.raw_scroll_delta.y);
            if scroll != 0.0 {
                let zoom_factor = (scroll * ZOOM_SPEED).exp();
                self.config.zoom = (self.config.zoom * zoom_factor).clamp(MIN_ZOOM, MAX_ZOOM);
            }
        }

        // Перетаскивание мышью
        if response.dragged() {
            self.config.pan += response.drag_delta();
        }

        let image_size = self.config.image_size * zoom;
        let rect = egui::Rect::from_min_size(
            response.rect.center() - image_size / 2.0 + self.config.pan,
            image_size,
        );

        painter.image(
            self.config.texture.id(),
            rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );

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
