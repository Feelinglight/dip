#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct AppConfig {
    pub tabs_state: String,
    // Путь к последней открытой картинки. Используется чтобы восстанавливать последний путь в
    // файловом менеджере
    pub last_image_path: String,
}
