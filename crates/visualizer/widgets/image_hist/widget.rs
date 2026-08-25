use egui::Widget;
use egui::{self, Vec2};
use egui_plot::{Bar, BarChart, Legend, Plot};

use super::state::ImageHistState;

use crate::widgets::transforms_panel::TransformsPanel;
use crate::widgets::zoom_texture::ZoomTexture;

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
                        self.state.reset_zoom_texture();
                        self.state.reload_image(ui.ctx());
                    }

                    ui.text_edit_singleline(&mut self.state.image_path_edit_text);

                    let clear_image_button =
                        egui::Button::image(egui::include_image!("../icons/clear-image.png"));
                    if ui.add(clear_image_button).clicked() {
                        self.state.reset_zoom_texture();
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
                            self.show_histogram(ui);
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

    fn show_histogram(&self, ui: &mut egui::Ui) {
        #[allow(clippy::cast_precision_loss, clippy::indexing_slicing)]
        let chart = BarChart::new(
            "Гистограмма изображения",
            (0..self.state.run.hist.len())
                .step_by(1)
                .map(|x| Bar::new(x as f64, self.state.run.hist[x]))
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
