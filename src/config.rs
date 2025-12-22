/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct AppConfig {
    pub hist_enable: bool,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            // Example stuff:
            hist_enable: false,
            value: 2.7,
        }
    }
}
