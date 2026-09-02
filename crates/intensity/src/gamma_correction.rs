use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Pixel, Primitive};
use num_traits::{NumCast, ToPrimitive};

#[derive(Clone, Debug, PartialEq)]
pub struct GammaCorrectionParams {
    constant: f64,
    gamma: f64,
}

impl Default for GammaCorrectionParams {
    fn default() -> Self {
        Self::new(1., 1.)
    }
}

impl GammaCorrectionParams {
    /// Creates gamma correction parameters.
    ///
    /// # Panics
    ///
    /// Panics if `constant` is less than `1.0` or `gamma` is not positive.
    #[must_use]
    pub fn new(constant: f64, gamma: f64) -> Self {
        assert!(constant >= 1., "Константа должна быть больше, либо равна 1");
        assert!(gamma > 0., "Коеффициенты гамма должен быть больше 0");
        Self { constant, gamma }
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

pub trait GammaCorrect: Sized {
    fn gamma_correct_inplace(&mut self, params: &GammaCorrectionParams);

    #[inline]
    #[must_use]
    fn gamma_correct_single(value: f64, params: &GammaCorrectionParams, max_value: f64) -> f64 {
        let powered = (value / max_value).powf(params.gamma);
        (params.constant * powered).min(1.) * max_value
    }

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
            let corrected = Self::gamma_correct_single(fsample, params, max_value).round();
            *sample = NumCast::from(corrected).unwrap_or(P::Subpixel::DEFAULT_MIN_VALUE);
        }
    }
}
