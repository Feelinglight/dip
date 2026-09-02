use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Pixel};
use intensity::{
    gamma_correction::{GammaCorrect, GammaCorrectionParams},
    log_transform::{LogTransform, LogTransformParams},
    negative::Negative,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TransformKind {
    Negative,
    GammaCorrection,
    LogTransform,
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Transform {
    Negative,
    GammaCorrection(#[serde(with = "GammaCorrectionParamsDef")] GammaCorrectionParams),
    LogTransform(#[serde(with = "LogTransformParamsDef")] LogTransformParams),
}

impl Transform {
    #[must_use]
    pub fn new(kind: TransformKind) -> Transform {
        match kind {
            TransformKind::Negative => Transform::Negative,
            TransformKind::GammaCorrection => {
                Transform::GammaCorrection(GammaCorrectionParams::default())
            }
            TransformKind::LogTransform => Transform::LogTransform(LogTransformParams::default()),
        }
    }

    #[must_use]
    pub const fn available_kinds() -> &'static [TransformKind] {
        &[
            TransformKind::Negative,
            TransformKind::GammaCorrection,
            TransformKind::LogTransform,
        ]
    }

    #[must_use]
    pub const fn kind(&self) -> TransformKind {
        match self {
            Transform::Negative => TransformKind::Negative,
            Transform::GammaCorrection(_) => TransformKind::GammaCorrection,
            Transform::LogTransform(_) => TransformKind::LogTransform,
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
            Transform::GammaCorrection(params) => {
                image_buffer.gamma_correct_inplace(params);
            }
            Transform::LogTransform(params) => {
                image_buffer.log_transform_inplace(params);
            }
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(remote = "GammaCorrectionParams")]
struct GammaCorrectionParamsDef {
    #[serde(getter = "GammaCorrectionParams::constant")]
    constant: f64,
    #[serde(getter = "GammaCorrectionParams::gamma")]
    gamma: f64,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(remote = "LogTransformParams")]
struct LogTransformParamsDef {
    #[serde(getter = "LogTransformParams::constant")]
    constant: f64,
    #[serde(getter = "LogTransformParams::log_base")]
    log_base: f64,
}

impl From<GammaCorrectionParamsDef> for GammaCorrectionParams {
    fn from(params: GammaCorrectionParamsDef) -> Self {
        Self::new(params.constant, params.gamma)
    }
}

impl From<LogTransformParamsDef> for LogTransformParams {
    fn from(params: LogTransformParamsDef) -> Self {
        Self::new(params.constant, params.log_base)
    }
}
