use std::path::Path;

use egui::{ColorImage, TextureHandle, Vec2, Widget};
use egui_plot::{Bar, BarChart, Legend, Plot};
use image::GrayImage;
use intensity::graduation::Negative;
use intensity::hist::{HistArray, empty_hist, make_hist};

use crate::ZoomTexture;
use crate::widgets::transforms_panel::{AppliedTransform, TransformsPanel};
use crate::widgets::zoom_texture::ZoomTextureState;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ImageHistState {
    zt_state: ZoomTextureState,
    transforms: Vec<AppliedTransform>,
    image_path_edit_text: String,
    hist_enable: bool,

    transforms_viewport_size: Option<egui::Vec2>,
    transforms_viewport_pos: Option<egui::Pos2>,

    #[serde(skip)]
    run: ImageHistRunState,
}

struct Images {
    pub original: GrayImage,
    pub transformed: GrayImage,
}

struct ImageHistRunState {
    images: Option<Images>,
    show_image_controls: bool,
    hist: HistArray,
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
    fn reload_image(&mut self, ctx: &egui::Context) {
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
            transform.restore_state();
        }
    }

    fn set_images(&mut self, ctx: &egui::Context, original_image: GrayImage) {
        let mut transformed = original_image.clone();

        Self::apply_active_transforms(&self.transforms, &mut transformed);
        self.zt_state
            .set_texture(load_egui_texture(ctx, &transformed), false);

        self.run.images = Some(Images {
            original: original_image,
            transformed,
        });
    }

    fn apply_active_transforms(transforms: &[AppliedTransform], image: &mut GrayImage) {
        for transform in transforms {
            if transform.is_active() {
                transform.apply(image);
            }
        }
    }

    fn apply_transforms(&mut self, ctx: &egui::Context) {
        if let Some(images) = &mut self.run.images {
            let mut new_transformed = images.original.clone();

            Self::apply_active_transforms(&self.transforms, &mut new_transformed);
            self.zt_state
                .set_texture(load_egui_texture(ctx, &new_transformed), false);

            images.transformed = new_transformed;
        }
    }

    /// Сбрасывает текущее загруженное изображение в его первоначальное состояние
    /// Если изображение не загружено, то не делает ничего
    fn reset_zoom_texture(&mut self) {
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

                    if ui.button("Преобразовать").clicked() {
                        self.state.run.show_image_controls = true;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.checkbox(&mut self.state.hist_enable, "Показать гистограмму");
                    });
                    resp
                });

                ui.separator();

                self.show_transforms_viewport(ui);

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

    fn show_transforms_viewport(&mut self, ui: &mut egui::Ui) {
        if self.state.run.show_image_controls {
            let mut viewport_builder = egui::ViewportBuilder::default()
                .with_title(format!(
                    "Преобразования для {}",
                    self.state.image_path_edit_text
                ))
                .with_inner_size(
                    self.state
                        .transforms_viewport_size
                        .unwrap_or(Vec2::new(800., 600.)),
                );

            // NOTE: Не работает на Wayland
            if let Some(viewport_pos) = self.state.transforms_viewport_pos {
                viewport_builder = viewport_builder.with_position(viewport_pos);
            }

            let transforms_viewport_id = ui.make_persistent_id("transforms_viewport");
            ui.ctx().show_viewport_immediate(
                egui::ViewportId::from_hash_of(transforms_viewport_id),
                viewport_builder,
                |ui, class| {
                    // NOTE: Не работает на Wayland
                    if let Some(inner_rect) = ui.input(|i| i.viewport().inner_rect) {
                        self.state.transforms_viewport_size = Some(inner_rect.size());
                    }
                    if let Some(outer_rect) = ui.input(|i| i.viewport().outer_rect) {
                        self.state.transforms_viewport_pos = Some(outer_rect.min);
                    }

                    let transforms_panel = TransformsPanel::new(&mut self.state.transforms);
                    if ui.add(transforms_panel).changed() {
                        self.state.apply_transforms(ui.ctx());
                    }

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
