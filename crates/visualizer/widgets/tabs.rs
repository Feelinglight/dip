use crate::widgets::image_hist::{ImageHist, ImageHistState};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ImageHistTab {
    pub id: egui::Id,
    pub state: ImageHistState,
}

impl Default for ImageHistTab {
    fn default() -> Self {
        Self {
            id: egui::Id::new(uuid::Uuid::new_v4()),
            state: ImageHistState::default(),
        }
    }
}

pub struct TabViewer<'a> {
    added_tabs: &'a mut Vec<(egui_dock::SurfaceIndex, egui_dock::NodeIndex)>,
}

impl<'a> TabViewer<'a> {
    pub fn new(added_tabs: &'a mut Vec<(egui_dock::SurfaceIndex, egui_dock::NodeIndex)>) -> Self {
        Self { added_tabs }
    }
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = ImageHistTab;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        tab.id
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.state.image_path().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let mut image_hist = ImageHist::new(tab.id, &mut tab.state);
        ui.add(&mut image_hist);
    }

    fn on_add(&mut self, surface: egui_dock::SurfaceIndex, node: egui_dock::NodeIndex) {
        self.added_tabs.push((surface, node));
    }

    fn on_close(&mut self, _tab: &mut Self::Tab) -> egui_dock::tab_viewer::OnCloseResponse {
        egui_dock::tab_viewer::OnCloseResponse::Ignore
    }

    // fn on_rect_changed(&mut self, _tab: &mut Self::Tab) {}
}
