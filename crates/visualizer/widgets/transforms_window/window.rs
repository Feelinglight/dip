use egui::{self, Panel};
use egui_ltreeview::TreeViewState;
use uuid::Uuid;

use crate::widgets::transforms_window::{
    applied_transforms_tree::show_applied_transforms,
    available_transforms_tree::show_available_transforms, data::AppliedTransform,
};

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct TranswormsWindowState {
    applied_transforms: Vec<AppliedTransform>,
    applied_transforms_tree_state: TreeViewState<Uuid>,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct TransformsWindow {
    state: TranswormsWindowState,
}

impl TransformsWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        Panel::left(egui::Id::new("all transforms"))
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

                show_applied_transforms(ui, &mut self.state.applied_transforms);

                // ScrollArea::both()
                // .scroll([
                //     false,
                //     self.settings.scroll_vertical,
                // ])
                // .scroll_bar_visibility(
                //     egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                // )
                // .show(ui, |ui| {
                //     self.show_tree_view(ui);
                // });
            });

        Panel::right(egui::Id::new("transforms"))
            .resizable(true)
            .min_size(180.)
            .show(ui, |ui| {
                ui.heading("Преобразования");
                ui.separator();

                show_available_transforms(ui, |transform_kind| {
                    self.state
                        .applied_transforms
                        .push(AppliedTransform::new(transform_kind));
                });
            });

        // Чтобы отступ сверху был таким же как у боковых панелей
        let custom_frame =
            egui::Frame::central_panel(ui.style()).inner_margin(egui::Margin::symmetric(8, 2));

        egui::CentralPanel::default()
            .frame(custom_frame)
            .show(ui, |ui| {
                ui.set_min_width(300.);
                ui.heading("Параметры преобразования");

                ui.separator();
                // if let Some(selected_control) =
                // self.state.applied_transforms_tree_state.selected().first()
                // {
                // self.controls_data.find_mut(selected_control, &mut |node| {
                // show_node_content(ui, node);
                // });
                // }
            });
    }
}
