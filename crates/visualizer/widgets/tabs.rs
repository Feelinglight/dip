use crate::widgets::image_hist::{ImageHist, ImageHistState};

pub struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = ImageHistState;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        tab.id()
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.image_path().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let mut image_hist = ImageHist::new(tab);
        ui.add(&mut image_hist);
    }

    fn on_rect_changed(&mut self, _tab: &mut Self::Tab) {}
}
