use std::path::Path;

use egui::{Context, Widget};
use egui_plot::{Bar, BarChart, Legend, Plot};
use image::{GrayImage, ImageReader};
use intensity::graduation::Negative;
use intensity::hist::{HistArray, make_option_hist};
use log::error;
use uuid::Uuid;

use crate::ZoomTexture;
use crate::{errors::LoadImageError, widgets::zoom_texture::ZoomTextureState};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ImageHistState {
    id: egui::Id,
    zt_state: ZoomTextureState,
    image_path_edit_text: String,
    hist_enable: bool,

    #[serde(skip)]
    image: Option<GrayImage>,
    #[serde(skip)]
    hist: HistArray,
}

impl Default for ImageHistState {
    fn default() -> Self {
        Self {
            id: egui::Id::new(Uuid::new_v4()),
            zt_state: ZoomTextureState::default(),
            image_path_edit_text: String::new(),
            image: None,
            hist: make_option_hist(None),
            hist_enable: true,
        }
    }
}

fn load_image(path: &Path) -> Result<GrayImage, LoadImageError> {
    Ok(ImageReader::open(path)?
        .with_guessed_format()?
        .decode()?
        .to_luma8())
}

impl ImageHistState {
    /// Инициализирует состояние
    /// Загружает изображение по текущему установленному пути и строит его гистограмму
    pub fn init(&mut self, egui_ctx: &Context) {
        self.reload_image(egui_ctx);
    }

    pub fn id(&self) -> egui::Id {
        self.id
    }

    pub fn image_path(&self) -> &String {
        &self.image_path_edit_text
    }

    // Загружает изображение по пути ``path``
    pub fn load_image(&mut self, ctx: &egui::Context, path: &Path) {
        if let Some(image_path) = path.to_str() {
            self.image_path_edit_text = String::from(image_path);
            self.reload_image(ctx);
        } else {
            error!("Ошибка. Не удалось загрузить изображение: путь содержит не UTF-8 символы");
        }
    }

    /// Загружает изображение по текущему установленному пути и строит его гистограмму
    pub fn reload_image(&mut self, ctx: &egui::Context) {
        let image = load_image(Path::new(&self.image_path_edit_text));
        self.zt_state.set_texture(ctx, image.as_ref(), false);
        self.image = image.ok();
        self.update_hist();
    }

    /// Сбрасывает текущее загруженное изображение в его первоначальное состояние
    /// Если изображение не загружено, то не делает ничего
    fn reset_zoom_texture(&mut self, ctx: &egui::Context) {
        if let Some(img) = &self.image {
            self.zt_state.set_texture(ctx, Ok(img), false);
        }
    }

    /// Строит гистограмму для текущего изображения
    fn update_hist(&mut self) {
        self.hist = make_option_hist(self.image.as_ref());
    }

    fn test_function(&mut self, ui: &mut egui::Ui) {
        if let Some(img) = &mut self.image {
            img.negative_inplace();
            self.update_hist();
            self.reset_zoom_texture(ui.ctx());
        }
    }

    fn show_histogram(&self, id: egui::Id, ui: &mut egui::Ui) {
        #[allow(clippy::cast_precision_loss, clippy::indexing_slicing)]
        let chart = BarChart::new(
            "Гистограмма изображения",
            (0..self.hist.len())
                .step_by(1)
                .map(|x| Bar::new(x as f64, self.hist[x]))
                .collect(),
        )
        .color(egui::Color32::LIGHT_BLUE);

        Plot::new(id)
            .legend(Legend::default())
            .clamp_grid(true)
            .allow_zoom(egui::Vec2b::new(true, true))
            .allow_drag(egui::Vec2b::new(true, true))
            .allow_scroll(egui::Vec2b::new(false, false))
            .show(ui, |plot_ui| plot_ui.bar_chart(chart));
    }
}

pub struct ImageHist<'a> {
    state: &'a mut ImageHistState,
    open_image_requested: bool,
}

impl<'a> ImageHist<'a> {
    pub fn new(state: &'a mut ImageHistState) -> ImageHist<'a> {
        Self {
            state,
            open_image_requested: false,
        }
    }
}

impl Widget for &mut ImageHist<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let id = ui.make_persistent_id(self.state.id);

        ui.vertical(|ui| {
            let open_image_button_resp = ui.horizontal(|ui| {
                let open_file_button =
                    egui::Button::image(egui::include_image!("../icons/open-file.png"));
                let resp = ui.add(open_file_button);

                let load_image_button =
                    egui::Button::image(egui::include_image!("../icons/load-image.png"));
                if ui.add(load_image_button).clicked() {
                    self.state.zt_state.reset_parameters();
                    self.state.reload_image(ui.ctx());
                }

                ui.text_edit_singleline(&mut self.state.image_path_edit_text);

                let clear_image_button =
                    egui::Button::image(egui::include_image!("../icons/clear-image.png"));
                if ui.add(clear_image_button).clicked() {
                    self.state.zt_state.reset_parameters();
                }

                if ui.button("Тест").clicked() {
                    self.state.test_function(ui);
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.checkbox(&mut self.state.hist_enable, "Показать гистограмму");
                });
                resp
            });

            ui.separator();

            if self.state.hist_enable {
                egui::SidePanel::right(id).show_inside(ui, |ui| {
                    ui.vertical(|ui| {
                        self.state.show_histogram(id, ui);
                    });
                });
            }

            let zoom_texture = ZoomTexture::new(&mut self.state.zt_state, ui.available_size());
            let zt_response = ui.add(zoom_texture);
            if open_image_button_resp.inner.clicked() || zt_response.double_clicked() {
                self.open_image_requested = true;
            }
            zt_response
        })
        .inner
    }
}

impl ImageHist<'_> {
    pub fn open_image_requested(&self) -> bool {
        self.open_image_requested
    }
}
