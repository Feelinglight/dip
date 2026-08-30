use std::path::Path;

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
