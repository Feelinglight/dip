use crate::widgets::image_hist::ImageHistState;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct AppConfig {
    pub image_states: Vec<ImageHistState>,
    pub tabs_state: String,
}
