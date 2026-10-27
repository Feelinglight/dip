use std::{
    error::Error,
    fmt,
    ops::{Deref, DerefMut},
};

use image::{ImageBuffer, Pixel, Primitive};
use num_traits::{NumCast, ToPrimitive};

#[derive(Debug)]
pub enum LogTransformError {
    InvalidConstant { constant: f64 },
    InvalidLogBase { log_base: f64 },
}

impl Error for LogTransformError {}

impl fmt::Display for LogTransformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogTransformError::InvalidConstant { constant } => {
                write!(
                    f,
                    "значение константы должно быть больше либо равно 1. Получено {constant}"
                )
            }
            LogTransformError::InvalidLogBase { log_base } => {
                write!(
                    f,
                    "основание логарифма должно быть больше 0 и не равно 1. Получено {log_base}"
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogTransformParams {
    constant: f64,
    log_base: f64,
}

impl Default for LogTransformParams {
    fn default() -> Self {
        Self::try_new(1., 0.99).expect("дефолтные значения параметров всегда должны быть валидны")
    }
}

impl LogTransformParams {
    /// # Errors
    ///
    /// Возвращает [`LogTransformError::InvalidConstant`], если ``constant`` меньше 1.
    /// Возвращает [`LogTransformError::InvalidLogBase`], если ``log_base`` меньше или равно 0,
    /// или равно 1.
    pub fn try_new(constant: f64, log_base: f64) -> Result<Self, LogTransformError> {
        if !constant.is_finite() || constant < 1. {
            Err(LogTransformError::InvalidConstant { constant })
        } else if !log_base.is_finite() || log_base <= 0. || (log_base - 1.).abs() < 0.000_001 {
            Err(LogTransformError::InvalidLogBase { log_base })
        } else {
            Ok(Self { constant, log_base })
        }
    }

    #[must_use]
    pub const fn constant(&self) -> f64 {
        self.constant
    }

    #[must_use]
    pub const fn log_base(&self) -> f64 {
        self.log_base
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[inline]
#[must_use]
pub fn log_transform_single(value: f64, params: &LogTransformParams, max_value: f64) -> f64 {
    // log_norm всегда от 0 до 1
    let log_norm = (value / max_value * (params.log_base - 1.) + 1.).log(params.log_base);
    max_value * (params.constant * log_norm).min(1.)
}

pub trait LogTransform: Sized {
    fn log_transform_inplace(&mut self, params: &LogTransformParams);

    #[must_use]
    fn log_transform(mut self, params: &LogTransformParams) -> Self {
        self.log_transform_inplace(params);
        self
    }
}

impl<P, Container> LogTransform for ImageBuffer<P, Container>
where
    P: Pixel,
    P::Subpixel: num_traits::PrimInt,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    fn log_transform_inplace(&mut self, params: &LogTransformParams) {
        let max_value = P::Subpixel::DEFAULT_MAX_VALUE.to_f64().unwrap_or(1.);

        for sample in self.iter_mut() {
            let fsample = (*sample).to_f64().unwrap_or(0.);
            let corrected = log_transform_single(fsample, params, max_value).round();
            *sample = NumCast::from(corrected).unwrap_or(P::Subpixel::DEFAULT_MIN_VALUE);
        }
    }
}
