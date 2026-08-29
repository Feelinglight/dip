use egui_dock::NodePath;

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

impl ImageHistTab {
    pub fn new(state: ImageHistState) -> Self {
        Self {
            id: egui::Id::new(uuid::Uuid::new_v4()),
            state,
        }
    }
}

pub struct TabViewer<'a> {
    tabs_count: usize,
    on_open_image: &'a mut dyn FnMut(NodePath),
}

impl<'a> TabViewer<'a> {
    pub fn new(tabs_count: usize, on_open_image: &'a mut dyn FnMut(NodePath)) -> Self {
        Self {
            tabs_count,
            on_open_image,
        }
    }
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = ImageHistTab;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        tab.id
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        let tab_image_path = tab.state.image_path();
        if let Some((_, suffix)) = tab_image_path.rsplit_once('/')
            && !suffix.is_empty()
            && tab_image_path.starts_with('/')
        {
            suffix
        } else {
            tab_image_path
        }
        .into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let mut image_hist = ImageHist::new(tab.id, &mut tab.state);
        ui.add(&mut image_hist);

        if image_hist.open_image_requested() {
            (self.on_open_image)(NodePath::right_node(NodePath::MAIN_ROOT));
        }
    }

    fn on_add(&mut self, path: NodePath) {
        (self.on_open_image)(path);
    }

    fn is_closeable(&self, _tab: &Self::Tab) -> bool {
        self.tabs_count > 1
    }
}
