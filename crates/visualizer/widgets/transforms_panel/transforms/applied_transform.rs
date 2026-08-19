use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Pixel};
use intensity::graduation::LogTransform;
use intensity::graduation::{GammaCorrect, Negative};
use uuid::Uuid;

use super::GammaCorrectionData;
use super::LogTransformData;

#[derive(Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize, Debug)]
pub enum TransformKind {
    Negative,
    GammaCorrection,
    LogTransform,
}

impl TransformKind {
    pub fn name(&self) -> &'static str {
        match self {
            TransformKind::Negative => "Негатив",
            TransformKind::GammaCorrection => "Гамма-коррекция",
            TransformKind::LogTransform => "Логарифм. преобраз.",
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum TransformParameters {
    Negative,
    GammaCorrection(GammaCorrectionData),
    LogTransform(LogTransformData),
}

impl TransformParameters {
    pub fn new(kind: &TransformKind) -> TransformParameters {
        match kind {
            TransformKind::Negative => TransformParameters::Negative,
            TransformKind::GammaCorrection => {
                TransformParameters::GammaCorrection(GammaCorrectionData::default())
            }
            TransformKind::LogTransform => {
                TransformParameters::LogTransform(LogTransformData::default())
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AppliedTransform {
    pub id: Uuid,
    pub kind: TransformKind,
    pub parameters: TransformParameters,
    active: bool,
}

impl AppliedTransform {
    pub fn new(kind: TransformKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            parameters: TransformParameters::new(&kind),
            kind,
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

    pub fn restore_state(&mut self) {
        match &mut self.parameters {
            TransformParameters::Negative => {}
            TransformParameters::GammaCorrection(data) => {
                data.restore();
            }
            TransformParameters::LogTransform(data) => {
                data.restore();
            }
        }
    }

    pub fn apply<P, Container>(&self, image_buffer: &mut ImageBuffer<P, Container>)
    where
        P: Pixel,
        Container: Deref<Target = [P::Subpixel]> + DerefMut,
    {
        match &self.parameters {
            TransformParameters::Negative => {
                image_buffer.negative_inplace();
            }
            TransformParameters::GammaCorrection(data) => {
                image_buffer.gamma_correct_inplace(data.gamma(), data.constant());
            }
            TransformParameters::LogTransform(data) => {
                image_buffer.log_transform_inplace(data.log_base(), data.constant());
            }
        }
    }
}
