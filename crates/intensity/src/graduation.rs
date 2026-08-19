use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Pixel, Primitive};
use num_traits::{NumCast, ToPrimitive};

pub trait Negative: Sized {
    fn negative_inplace(&mut self);

    fn negative(mut self) -> Self {
        self.negative_inplace();
        self
    }
}

impl<P, Container> Negative for ImageBuffer<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    fn negative_inplace(&mut self) {
        for sample in self.iter_mut() {
            *sample = P::Subpixel::DEFAULT_MAX_VALUE - *sample;
        }
    }
}

pub trait GammaCorrect: Sized {
    fn gamma_correct_inplace(&mut self, gamma: f64, constant: f64);

    fn gamma_correct(mut self, gamma: f64, constant: f64) -> Self {
        self.gamma_correct_inplace(gamma, constant);
        self
    }
}

impl<P, Container> GammaCorrect for ImageBuffer<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    fn gamma_correct_inplace(&mut self, gamma: f64, constant: f64) {
        assert!(constant >= 1., "Константа должна быть больше, либо равна 1");
        assert!(gamma > 0., "Коеффициенты гамма должен быть больше 0");

        let max_value = P::Subpixel::DEFAULT_MAX_VALUE.to_f64().unwrap_or(1.);

        for sample in self.iter_mut() {
            let fsample = (*sample).to_f64().unwrap_or(0.);
            let corrected = gamma_correct_single(fsample, gamma, constant, max_value).round();
            *sample = NumCast::from(corrected).unwrap_or(P::Subpixel::DEFAULT_MIN_VALUE);
        }
    }
}

#[inline]
pub fn gamma_correct_single(value: f64, gamma: f64, constant: f64, max_value: f64) -> f64 {
    let powered = (value / max_value).powf(gamma);
    (constant * powered).min(1.) * max_value
}

pub trait LogTransform: Sized {
    fn log_transform_inplace(&mut self, gamma: f64, constant: f64);

    fn log_transform(mut self, gamma: f64, constant: f64) -> Self {
        self.log_transform_inplace(gamma, constant);
        self
    }
}

impl<P, Container> LogTransform for ImageBuffer<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    fn log_transform_inplace(&mut self, base: f64, constant: f64) {
        assert!(constant >= 1., "Константа должна быть больше, либо равна 1");
        assert!(base > 0., "Основание логарифма должно быть больше 0");

        let max_value = P::Subpixel::DEFAULT_MAX_VALUE.to_f64().unwrap_or(1.);

        for sample in self.iter_mut() {
            let fsample = (*sample).to_f64().unwrap_or(0.);
            let corrected = log_transform_single(fsample, base, constant, max_value).round();
            *sample = NumCast::from(corrected).unwrap_or(P::Subpixel::DEFAULT_MIN_VALUE);
        }
    }
}

#[inline]
pub fn log_transform_single(value: f64, base: f64, constant: f64, max_value: f64) -> f64 {
    constant * max_value * (value / max_value * (base - 1.) + 1.).log(base)
}
