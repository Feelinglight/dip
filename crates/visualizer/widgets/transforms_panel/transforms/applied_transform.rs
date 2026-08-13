use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Pixel};
use intensity::graduation::Negative;
use uuid::Uuid;

use super::GammaCorrectionData;

#[derive(Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize, Debug)]
pub enum TransformKind {
    Negative,
    GammaCorrection,
}

impl TransformKind {
    pub fn name(&self) -> &'static str {
        match self {
            TransformKind::Negative => "Негатив",
            TransformKind::GammaCorrection => "Гамма-коррекция",
        }
    }

    pub fn default_parameters(&self) -> TransformParameters {
        match self {
            TransformKind::Negative => TransformParameters::Negative,
            TransformKind::GammaCorrection => TransformParameters::gamma_correction(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum TransformParameters {
    Negative,
    GammaCorrection(GammaCorrectionData),
}

impl TransformParameters {
    pub fn gamma_correction() -> Self {
        TransformParameters::GammaCorrection(GammaCorrectionData::default())
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
            parameters: kind.default_parameters(),
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
            TransformParameters::GammaCorrection(data) => {
                data.restore();
            }
            TransformParameters::Negative => {}
        }
    }

    pub fn apply<P, Container>(&self, image_buffer: &mut ImageBuffer<P, Container>)
    where
        P: Pixel,
        Container: Deref<Target = [P::Subpixel]> + DerefMut,
    {
        match &self.parameters {
            TransformParameters::GammaCorrection(data) => {
                //
            }
            TransformParameters::Negative => {
                image_buffer.negative_inplace();
            }
        };
    }
}
