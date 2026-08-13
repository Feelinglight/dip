use image::GrayImage;

const HIST_ARRAY_SIZE: usize = u8::MAX as usize + 1;

pub type HistArray = [f64; HIST_ARRAY_SIZE];

pub fn make_hist(image: &GrayImage) -> HistArray {
    let mut hist = [0.; HIST_ARRAY_SIZE];
    for sample in image.as_flat_samples().samples.iter() {
        if let Some(elem) = hist.get_mut(*sample as usize) {
            *elem += 1.
        }
    }
    hist
}

pub fn empty_hist() -> HistArray {
    [0.; HIST_ARRAY_SIZE]
}
