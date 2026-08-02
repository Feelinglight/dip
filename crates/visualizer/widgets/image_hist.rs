use std::path::Path;

use egui::Widget;
use egui_plot::{Bar, BarChart, Legend, Plot};
use image::{GrayImage, ImageReader};
use intensity::graduation::Negative;
use intensity::hist::{HistArray, make_option_hist};

use crate::ZoomTexture;
use crate::widgets::transforms_window::{TransformsPanel, TransformsState};
use crate::{errors::LoadImageError, widgets::zoom_texture::ZoomTextureState};

struct ImageHistRunState {
    image: Option<GrayImage>,
    show_image_controls: bool,
    hist: HistArray,
}

impl Default for ImageHistRunState {
    fn default() -> Self {
        Self {
            image: None,
            show_image_controls: false,
            hist: make_option_hist(None),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ImageHistState {
    zt_state: ZoomTextureState,
    transforms: TransformsState,
    image_path_edit_text: String,
    hist_enable: bool,

    #[serde(skip)]
    run: ImageHistRunState,
}

impl Default for ImageHistState {
    fn default() -> Self {
        Self {
            zt_state: ZoomTextureState::default(),
            transforms: TransformsState::default(),
            image_path_edit_text: String::new(),
            hist_enable: true,
            run: ImageHistRunState::default(),
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
    pub fn image_path(&self) -> &String {
        &self.image_path_edit_text
    }

    /// Загружает изображение из массива данных ``data``. Устанавливает путь к изображению в
    /// ``path``.
    /// Путь к изображению требуется для повторной загрузки изображения с помощью метода
    /// ``reload_image``
    pub fn from_memory(
        ctx: &egui::Context,
        path: &Path,
        data: &[u8],
    ) -> Result<Self, &'static str> {
        if let Some(image_path) = path.to_str() {
            if let Ok(image) = image::load_from_memory(data) {
                let gray_image: GrayImage = image.to_luma8();

                let mut zt_state = ZoomTextureState::default();
                zt_state.set_texture(ctx, Ok(&gray_image), false);

                let mut state = Self {
                    zt_state,
                    image_path_edit_text: String::from(image_path),
                    run: ImageHistRunState {
                        image: Some(gray_image),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                state.update_hist();
                Ok(state)
            } else {
                Err("Ошибка. Не удалось загрузить изображение")
            }
        } else {
            Err("Ошибка. Не удалось загрузить изображение: путь содержит не UTF-8 символы")
        }
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
        let image = load_image(Path::new(&self.image_path_edit_text));
        self.zt_state.set_texture(ctx, image.as_ref(), false);
        self.run.image = image.ok();
        self.update_hist();
    }

    /// Сбрасывает текущее загруженное изображение в его первоначальное состояние
    /// Если изображение не загружено, то не делает ничего
    fn reset_zoom_texture(&mut self, ctx: &egui::Context) {
        if let Some(img) = &self.run.image {
            self.zt_state.set_texture(ctx, Ok(img), false);
        }
    }

    /// Строит гистограмму для текущего изображения
    fn update_hist(&mut self) {
        self.run.hist = make_option_hist(self.run.image.as_ref());
    }

    fn test_function(&mut self, ui: &mut egui::Ui) {
        if let Some(img) = &mut self.run.image {
            img.negative_inplace();
            self.update_hist();
            self.reset_zoom_texture(ui.ctx());
        }
    }

    fn show_histogram(&self, ui: &mut egui::Ui) {
        #[allow(clippy::cast_precision_loss, clippy::indexing_slicing)]
        let chart = BarChart::new(
            "Гистограмма изображения",
            (0..self.run.hist.len())
                .step_by(1)
                .map(|x| Bar::new(x as f64, self.run.hist[x]))
                .collect(),
        )
        .color(egui::Color32::LIGHT_BLUE);

        Plot::new("ImageHist::histogram")
            .legend(Legend::default())
            .clamp_grid(true)
            .allow_zoom(egui::Vec2b::new(true, true))
            .allow_drag(egui::Vec2b::new(true, true))
            .allow_scroll(egui::Vec2b::new(false, false))
            .show(ui, |plot_ui| plot_ui.bar_chart(chart));
    }
}

pub struct ImageHist<'a> {
    id_salt: egui::IdSalt,
    state: &'a mut ImageHistState,
    open_image_requested: bool,
}

impl<'a> ImageHist<'a> {
    pub fn new(id_salt: impl egui::AsIdSalt, state: &'a mut ImageHistState) -> ImageHist<'a> {
        Self {
            id_salt: egui::IdSalt::new(id_salt),
            state,
            open_image_requested: false,
        }
    }
}

impl Widget for &mut ImageHist<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.push_id(self.id_salt, |ui| {
            ui.vertical(|ui| {
                let toolbar_response = ui.horizontal_wrapped(|ui| {
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

                    if ui.button("Преобразовать").clicked() {
                        self.state.run.show_image_controls = true;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.checkbox(&mut self.state.hist_enable, "Показать гистограмму");
                    });
                    resp
                });

                ui.separator();

                self.show_controls_viewport(ui);

                let id = ui.make_persistent_id("ImageHist-right_panel");
                if self.state.hist_enable {
                    egui::Panel::right(id).show(ui, |ui| {
                        ui.vertical(|ui| {
                            self.state.show_histogram(ui);
                        });
                    });
                }

                let zoom_texture = ZoomTexture::new(&mut self.state.zt_state, ui.available_size());
                let zt_response = ui.add(zoom_texture);
                if toolbar_response.inner.clicked() || zt_response.double_clicked() {
                    self.open_image_requested = true;
                }
                zt_response
            })
            .inner
        })
        .inner
    }
}

impl ImageHist<'_> {
    pub fn open_image_requested(&mut self) -> bool {
        let requested = self.open_image_requested;
        self.open_image_requested = false;
        requested
    }

    fn show_controls_viewport(&mut self, ui: &mut egui::Ui) {
        if self.state.run.show_image_controls {
            let transforms_viewport_id = ui.make_persistent_id("transforms_viewport");
            ui.ctx().show_viewport_immediate(
                egui::ViewportId::from_hash_of(transforms_viewport_id),
                egui::ViewportBuilder::default()
                    .with_title(format!(
                        "Преобразования для {}",
                        self.state.image_path_edit_text
                    ))
                    .with_inner_size([800.0, 600.0]),
                |ui, class| {
                    let transforms_panel = TransformsPanel::new(&mut self.state.transforms);
                    ui.add(transforms_panel);

                    if class == egui::ViewportClass::EmbeddedWindow {
                    } else {
                        egui::CentralPanel::default().show(ui, |ui| {
                            if ui.input(|i| i.viewport().close_requested()) {
                                self.state.run.show_image_controls = false;
                            }
                        });
                    }
                },
            );
        }
    }
}
