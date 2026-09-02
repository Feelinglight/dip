use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Pixel, Primitive};
use num_traits::{NumCast, ToPrimitive};

#[derive(Clone, Debug, PartialEq)]
pub struct LogTransformParams {
    constant: f64,
    log_base: f64,
}

impl Default for LogTransformParams {
    fn default() -> Self {
        Self::new(1., 0.99)
    }
}

impl LogTransformParams {
    /// Creates log transform parameters.
    ///
    /// # Panics
    ///
    /// Panics if `constant` is less than `1.0`, or if `log_base` is not
    /// positive or is equal to `1.0`.
    #[must_use]
    pub fn new(constant: f64, log_base: f64) -> Self {
        assert!(constant >= 1., "Константа должна быть больше, либо равна 1");
        assert!(
            log_base > 0. && (log_base - 1.).abs() > 0.000_001,
            "Основание логарифма должно быть больше 0 и не равно 1"
        );
        Self { constant, log_base }
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

pub trait LogTransform: Sized {
    fn log_transform_inplace(&mut self, params: &LogTransformParams);

    #[inline]
    #[must_use]
    fn log_transform_single(value: f64, params: &LogTransformParams, max_value: f64) -> f64 {
        // log_norm всегда будет от 0 до 1
        let log_norm = (value / max_value * (params.log_base - 1.) + 1.).log(params.log_base);
        max_value * (params.constant * log_norm).min(1.)
    }

    #[must_use]
    fn log_transform(mut self, params: &LogTransformParams) -> Self {
        self.log_transform_inplace(params);
        self
    }
}

impl<P, Container> LogTransform for ImageBuffer<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    fn log_transform_inplace(&mut self, params: &LogTransformParams) {
        let max_value = P::Subpixel::DEFAULT_MAX_VALUE.to_f64().unwrap_or(1.);

        for sample in self.iter_mut() {
            let fsample = (*sample).to_f64().unwrap_or(0.);
            let corrected = Self::log_transform_single(fsample, params, max_value).round();
            *sample = NumCast::from(corrected).unwrap_or(P::Subpixel::DEFAULT_MIN_VALUE);
        }
    }
}
