use image::GrayImage;

const HIST_ARRAY_SIZE: usize = u8::MAX as usize + 1;

pub type Histogram = [u64; HIST_ARRAY_SIZE];

#[must_use]
pub fn histogram(image: &GrayImage) -> Histogram {
    let mut hist = [0; HIST_ARRAY_SIZE];
    for sample in image.as_flat_samples().samples {
        if let Some(elem) = hist.get_mut(*sample as usize) {
            *elem += 1;
        }
    }
    hist
}

#[must_use]
pub fn empty_histogram() -> Histogram {
    [0; HIST_ARRAY_SIZE]
}
