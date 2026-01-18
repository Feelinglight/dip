use crate::widgets::zoom_texture::ZoomTextureState;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppConfig {
    pub hist_enable: bool,

    pub zt_state: ZoomTextureState,

    #[serde(skip)]
    value: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            // Example stuff:
            hist_enable: false,
            zt_state: Default::default(),
            value: 2.7,
        }
    }
}
