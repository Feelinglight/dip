use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Pixel, Primitive};

pub trait Negative: Sized {
    fn negative_inplace(&mut self);

    #[must_use]
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
