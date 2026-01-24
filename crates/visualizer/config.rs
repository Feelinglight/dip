use crate::widgets::zoom_texture::ZoomTextureState;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct AppConfig {
    pub image_path_edit_text: String,
    pub hist_enable: bool,

    pub zt_state: ZoomTextureState,
}
