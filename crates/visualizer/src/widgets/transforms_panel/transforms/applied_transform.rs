use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Pixel};
use intensity::graduation::LogTransform;
use intensity::graduation::{GammaCorrect, Negative};
use uuid::Uuid;

use super::GammaCorrectionData;
use super::LogTransformData;

pub use super::gamma::show_gamma_controls;
pub use super::log_transform::show_log_transform_controls;

#[derive(Clone, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize, Debug)]
pub enum TransformKind {
    Negative,
    GammaCorrection,
    LogTransform,
}

impl TransformKind {
    pub fn name(self) -> &'static str {
        match self {
            TransformKind::Negative => "Негатив",
            TransformKind::GammaCorrection => "Гамма-коррекция",
            TransformKind::LogTransform => "Логарифм. преобраз.",
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Transform {
    Negative,
    GammaCorrection(GammaCorrectionData),
    LogTransform(LogTransformData),
}

impl Transform {
    pub fn new(kind: TransformKind) -> Transform {
        match kind {
            TransformKind::Negative => Transform::Negative,
            TransformKind::GammaCorrection => {
                Transform::GammaCorrection(GammaCorrectionData::default())
            }
            TransformKind::LogTransform => Transform::LogTransform(LogTransformData::default()),
        }
    }

    pub fn available_kinds() -> &'static [TransformKind] {
        &[
            TransformKind::Negative,
            TransformKind::GammaCorrection,
            TransformKind::LogTransform,
        ]
    }

    pub fn kind(&self) -> TransformKind {
        match self {
            Transform::Negative => TransformKind::Negative,
            Transform::GammaCorrection(_) => TransformKind::GammaCorrection,
            Transform::LogTransform(_) => TransformKind::LogTransform,
        }
    }

    pub fn restore_state(&mut self) {
        match self {
            Transform::Negative => {}
            Transform::GammaCorrection(data) => {
                data.restore();
            }
            Transform::LogTransform(data) => {
                data.restore();
            }
        }
    }

    pub fn apply<P, Container>(&self, image_buffer: &mut ImageBuffer<P, Container>)
    where
        P: Pixel,
        Container: Deref<Target = [P::Subpixel]> + DerefMut,
    {
        match self {
            Transform::Negative => {
                image_buffer.negative_inplace();
            }
            Transform::GammaCorrection(data) => {
                image_buffer.gamma_correct_inplace(data.gamma(), data.constant());
            }
            Transform::LogTransform(data) => {
                image_buffer.log_transform_inplace(data.log_base(), data.constant());
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AppliedTransform {
    pub id: Uuid,
    pub op: Transform,
    active: bool,
}

impl AppliedTransform {
    pub fn new(kind: TransformKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            op: Transform::new(kind),
            active: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn toggle_active(&mut self) {
        self.active = !self.active;
    }

    pub fn apply_if_active<P, Container>(&self, image_buffer: &mut ImageBuffer<P, Container>)
    where
        P: Pixel,
        Container: Deref<Target = [P::Subpixel]> + DerefMut,
    {
        if self.active {
            self.op.apply(image_buffer);
        }
    }
}

pub fn show_transform_controls(ui: &mut egui::Ui, transform: &mut Transform, changed: &mut bool) {
    match transform {
        Transform::Negative => {
            ui.label("Параметры отсутствуют");
        }
        Transform::GammaCorrection(gamma_data) => {
            show_gamma_controls(ui, gamma_data, changed);
        }
        Transform::LogTransform(log_transform_data) => {
            show_log_transform_controls(ui, log_transform_data, changed);
        }
    }
}
