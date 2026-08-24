use crate::widgets::image_hist::ImageHistState;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct AppConfig {
    pub tabs_state: String,
    // Путь к папке последней открытой картинки. Используется чтобы восстанавливать последний путь в
    // файловом менеджере
    pub last_image_path: String,
}
