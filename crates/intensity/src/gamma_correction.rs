use std::{
    error::Error,
    fmt::{self},
    ops::{Deref, DerefMut},
};

use image::{ImageBuffer, Pixel, Primitive};
use num_traits::{NumCast, ToPrimitive};

#[derive(Debug)]
pub enum GammaCorrectionError {
    InvalidConstant { constant: f64 },
    InvalidGamma { gamma: f64 },
}

impl Error for GammaCorrectionError {}

impl fmt::Display for GammaCorrectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GammaCorrectionError::InvalidConstant { constant } => write!(
                f,
                "значение константы должно быть больше либо равно 1. Получено {constant}",
            ),
            GammaCorrectionError::InvalidGamma { gamma } => {
                write!(
                    f,
                    "значение гамма должно быть больше нуля. Получено {gamma}",
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaCorrectionParams {
    constant: f64,
    gamma: f64,
}

impl Default for GammaCorrectionParams {
    fn default() -> Self {
        Self::try_new(1., 1.).expect("дефолтные значения параметров всегда должны быть валидны")
    }
}

impl GammaCorrectionParams {
    /// # Errors
    ///
    /// Возвращает [`GammaCorrectionError::InvalidConstant`], если ``constant`` меньше 1.
    /// Возвращает [`GammaCorrectionError::InvalidGamma`], если ``gamma`` меньше
    /// либо равно 0
    pub fn try_new(constant: f64, gamma: f64) -> Result<Self, GammaCorrectionError> {
        if !constant.is_finite() || constant < 1. {
            Err(GammaCorrectionError::InvalidConstant { constant })
        } else if !gamma.is_finite() || gamma <= 0. {
            Err(GammaCorrectionError::InvalidGamma { gamma })
        } else {
            Ok(Self { constant, gamma })
        }
    }

    #[must_use]
    pub const fn constant(&self) -> f64 {
        self.constant
    }

    #[must_use]
    pub const fn gamma(&self) -> f64 {
        self.gamma
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[inline]
#[must_use]
pub fn gamma_correct_single(value: f64, params: &GammaCorrectionParams, max_value: f64) -> f64 {
    let powered = (value / max_value).powf(params.gamma);
    (params.constant * powered).min(1.) * max_value
}

pub trait GammaCorrect: Sized {
    fn gamma_correct_inplace(&mut self, params: &GammaCorrectionParams);

    #[must_use]
    fn gamma_correct(mut self, params: &GammaCorrectionParams) -> Self {
        self.gamma_correct_inplace(params);
        self
    }
}

impl<P, Container> GammaCorrect for ImageBuffer<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    fn gamma_correct_inplace(&mut self, params: &GammaCorrectionParams) {
        let max_value = P::Subpixel::DEFAULT_MAX_VALUE.to_f64().unwrap_or(1.);

        for sample in self.iter_mut() {
            let fsample = (*sample).to_f64().unwrap_or(0.);
            let corrected = gamma_correct_single(fsample, params, max_value).round();
            *sample = NumCast::from(corrected).unwrap_or(P::Subpixel::DEFAULT_MIN_VALUE);
        }
    }
}
