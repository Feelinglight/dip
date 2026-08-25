use std::path::Path;

use egui::{ColorImage, TextureHandle};
use image::GrayImage;

pub(super) fn load_gray_image(path: &Path, image_data: Option<&[u8]>) -> Result<GrayImage, String> {
    let image_path = path.to_str().ok_or(
        "Ошибка. Не удалось загрузить изображение: путь содержит не UTF-8 символы".to_string(),
    )?;

    let image = if let Some(data) = image_data {
        image::load_from_memory(data)
    } else {
        image::open(image_path)
    }
    .map_err(|err| err.to_string())?;

    Ok(image.to_luma8())
}

pub(super) fn load_texture(ctx: &egui::Context, image: &GrayImage) -> TextureHandle {
    let colored_image = ColorImage::from_gray(
        [image.width() as usize, image.height() as usize],
        image.as_flat_samples().as_slice(),
    );
    ctx.load_texture("dip", colored_image, egui::TextureOptions::LINEAR)
}
