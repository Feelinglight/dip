use super::state::ImageHistState;
use crate::widgets::transforms_panel::TransformsPanel;
use crate::widgets::zoom_texture::ZoomTexture;
use egui::Vec2;
use egui::Widget;
use egui_plot::{Bar, BarChart, Legend, Plot};
use intensity::histogram::HistArray;

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
                self.show_toolbar(ui);

                ui.separator();

                self.show_viewport(ui);

                let id = ui.make_persistent_id("ImageHist-right_panel");
                if self.state.histogram_enabled() {
                    egui::Panel::right(id).show(ui, |ui| {
                        ui.vertical(|ui| {
                            ImageHist::show_histogram(ui, self.state.histogram());
                        });
                    });
                }

                let zoom_texture =
                    ZoomTexture::new(self.state.zoom_texture_state_mut(), ui.available_size());
                let zt_response = ui.add(zoom_texture);

                if self.open_image_requested || zt_response.double_clicked() {
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

    fn show_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            let open_file_button =
                egui::Button::image(egui::include_image!("../icons/open-file.png"));
            if ui
                .add(open_file_button)
                .on_hover_text("Открыть изображение")
                .clicked()
            {
                self.open_image_requested = true;
            }

            let load_image_button =
                egui::Button::image(egui::include_image!("../icons/load-image.png"));
            if ui
                .add(load_image_button)
                .on_hover_text("Перезагрузить изображение")
                .clicked()
            {
                self.state.reset_zoom_texture();
                self.state.reload_image(ui.ctx());
            }

            ui.text_edit_singleline(self.state.image_path_mut());

            let clear_image_button =
                egui::Button::image(egui::include_image!("../icons/clear-image.png"));
            if ui
                .add(clear_image_button)
                .on_hover_text("Сбросить масштаб и сдвиг")
                .clicked()
            {
                self.state.reset_zoom_texture();
            }

            if ui.button("Преобразования").clicked() {
                self.state.open_transforms_viewport();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let switch_hist_button =
                    egui::Button::image(egui::include_image!("../icons/switch-hist.png"))
                        .selected(self.state.histogram_enabled());
                if ui
                    .add(switch_hist_button)
                    .on_hover_text("Показать или скрыть гистограмму")
                    .clicked()
                {
                    self.state.toggle_histogram();
                }
            });
        });
    }

    fn show_histogram(ui: &mut egui::Ui, hist: &HistArray) {
        #[allow(clippy::cast_precision_loss, clippy::indexing_slicing)]
        let chart = BarChart::new(
            "Гистограмма изображения",
            hist.iter()
                .enumerate()
                .map(|(x, &y)| Bar::new(x as f64, y))
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

    fn show_viewport(&mut self, ui: &mut egui::Ui) {
        if !self.state.transforms_viewport_open() {
            return;
        }

        let mut viewport_builder = egui::ViewportBuilder::default()
            .with_title(format!("Преобразования для {}", self.state.image_path()))
            .with_inner_size(
                self.state
                    .transforms_viewport_size()
                    .unwrap_or(Vec2::new(800., 600.)),
            );

        // NOTE: Не работает на Wayland
        if let Some(viewport_pos) = self.state.transforms_viewport_pos() {
            viewport_builder = viewport_builder.with_position(viewport_pos);
        }

        let transforms_viewport_id = ui.make_persistent_id("transforms_viewport");
        ui.ctx().show_viewport_immediate(
            egui::ViewportId::from_hash_of(transforms_viewport_id),
            viewport_builder,
            |ui, class| {
                // NOTE: Не работает на Wayland
                if let Some(inner_rect) = ui.input(|i| i.viewport().inner_rect) {
                    self.state.set_transforms_viewport_size(inner_rect.size());
                }
                if let Some(outer_rect) = ui.input(|i| i.viewport().outer_rect) {
                    self.state.set_transforms_viewport_pos(outer_rect.min);
                }

                let transforms_panel = TransformsPanel::new(self.state.transforms_mut());
                if ui.add(transforms_panel).changed() {
                    self.state.apply_transforms(ui.ctx());
                }

                if class != egui::ViewportClass::EmbeddedWindow {
                    egui::CentralPanel::default().show(ui, |ui| {
                        if ui.input(|i| i.viewport().close_requested()) {
                            self.state.close_transforms_viewport();
                        }
                    });
                }
            },
        );
    }
}
