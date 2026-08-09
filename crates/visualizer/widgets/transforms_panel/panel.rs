use egui::{Panel, ScrollArea, Widget};

use super::transforms::show_gamma_controls;

use super::applied_transforms_tree::show_applied_transforms;
use super::available_transforms_tree::show_available_transforms;
use super::transforms::AppliedTransform;
use super::transforms::TransformParameters;

pub struct TransformsPanel<'a> {
    id_salt: Option<egui::IdSalt>,
    transforms: &'a mut Vec<AppliedTransform>,
}

impl<'a> TransformsPanel<'a> {
    pub fn new(transforms: &'a mut Vec<AppliedTransform>) -> Self {
        Self {
            id_salt: None,
            transforms,
        }
    }
}

impl Widget for TransformsPanel<'_> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let id = ui.make_persistent_id(
            self.id_salt
                .unwrap_or_else(|| egui::IdSalt::new("transforms_panel")),
        );

        ui.push_id(id, |ui| {
            let left_id = ui.make_persistent_id("left_panel");
            let left_response = Panel::left(egui::Id::new(left_id))
                .resizable(true)
                .min_size(180.)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Применено");

                        let clear_image_button = egui::Button::image(egui::include_image!(
                            "../../icons/clear-image.png"
                        ));
                        if ui.add(clear_image_button).clicked() {
                            self.transforms.clear();
                        }
                    });

                    ui.separator();

                    show_applied_transforms(ui, self.transforms)
                })
                .response;

            let right_id = ui.make_persistent_id("right_panel");
            let right_response = Panel::right(egui::Id::new(right_id))
                .resizable(true)
                .min_size(180.)
                .show(ui, |ui| {
                    ui.heading("Преобразования");
                    ui.separator();

                    show_available_transforms(ui, |transform_kind| {
                        self.transforms.push(AppliedTransform::new(transform_kind));
                    })
                })
                .response;

            // Чтобы отступ сверху был таким же как у боковых панелей
            let custom_frame =
                egui::Frame::central_panel(ui.style()).inner_margin(egui::Margin::symmetric(8, 2));

            let central_response = egui::CentralPanel::default()
                .frame(custom_frame)
                .show(ui, |ui| {
                    ui.heading("Параметры преобразования");

                    ui.separator();

                    ScrollArea::both()
                        .id_salt("TransformsPanel::CentralPanel")
                        .auto_shrink([false, false])
                        .scroll([true, true])
                        .scroll_bar_visibility(
                            egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                        )
                        .show(ui, |ui| {
                            self.show_applied_transforms(ui);
                        })
                })
                .response;

            left_response | right_response | central_response
        })
        .inner
    }
}

impl TransformsPanel<'_> {
    fn show_applied_transforms(&mut self, ui: &mut egui::Ui) {
        for (idx, transform) in self.transforms.iter_mut().enumerate() {
            ui.push_id(transform.id, |ui| {
                ui.add_enabled_ui(transform.is_active(), |ui| {
                    ui.heading(format!("{}. {}", idx + 1, transform.kind.name()));

                    match &mut transform.parameters {
                        TransformParameters::Negative => {
                            ui.label("Параметры отсутствуют");
                        }
                        TransformParameters::GammaCorrection(gamma_data) => {
                            show_gamma_controls(ui, gamma_data);
                        }
                    }
                });
                ui.separator();
            });
        }
    }
}
