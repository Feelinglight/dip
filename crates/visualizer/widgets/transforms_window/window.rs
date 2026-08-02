use egui::{Panel, ScrollArea, Widget};
use egui_ltreeview::TreeViewState;
use uuid::Uuid;

use crate::widgets::transforms_window::{
    applied_transforms_tree::show_applied_transforms,
    available_transforms_tree::show_available_transforms, data::AppliedTransform,
};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct TransformsState {
    applied_transforms: Vec<AppliedTransform>,
    applied_transforms_tree_state: TreeViewState<Uuid>,
}

pub struct TransformsPanel<'a> {
    state: &'a mut TransformsState,
}

impl<'a> TransformsPanel<'a> {
    pub fn new(state: &'a mut TransformsState) -> Self {
        Self { state }
    }
}

impl Widget for TransformsPanel<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let left_response = Panel::left(egui::Id::new("all transforms"))
            .resizable(true)
            .min_size(180.)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Применено");

                    let clear_image_button =
                        egui::Button::image(egui::include_image!("../../icons/clear-image.png"));
                    if ui.add(clear_image_button).clicked() {
                        self.state.applied_transforms.clear();
                    }
                });

                ui.separator();

                show_applied_transforms(ui, &mut self.state.applied_transforms)
            })
            .response;

        let right_response = Panel::right(egui::Id::new("transforms"))
            .resizable(true)
            .min_size(180.)
            .show(ui, |ui| {
                ui.heading("Преобразования");
                ui.separator();

                show_available_transforms(ui, |transform_kind| {
                    self.state
                        .applied_transforms
                        .push(AppliedTransform::new(transform_kind));
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
                        for (idx, transform) in
                            &mut self.state.applied_transforms.iter_mut().enumerate()
                        {
                            ui.add_enabled_ui(transform.is_active(), |ui| {
                                ui.heading(format!("{}. {}", idx + 1, transform.kind.name()));

                                match &mut transform.parameters {
                                    super::data::TransformParameters::Negative => {
                                        ui.label("Параметры отсутствуют");
                                    }
                                    super::data::TransformParameters::GammaCorrection(gamma) => {
                                        ui.horizontal(|ui| {
                                            ui.label("Гамма:");
                                            ui.add(egui::Slider::new(gamma, 1.0..=100.));
                                        });
                                    }
                                }
                            });
                            ui.separator();
                        }
                    })
            })
            .response;

        left_response | right_response | central_response
    }
}
