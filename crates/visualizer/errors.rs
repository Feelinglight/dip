use core::fmt;
use std::error::Error;
use std::io;

use image::ImageError;

#[derive(Debug)]
pub enum LoadImageError {
    IoError(io::Error),
    DecodeImageError(ImageError),
}

impl fmt::Display for LoadImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadImageError::IoError(err) => {
                write!(f, "Ошибка IO: \"{}\"", err)
            }
            LoadImageError::DecodeImageError(err) => {
                write!(f, "Не удалось декодировать изображение: \"{}\"", err)
            }
        }
    }
}

impl Error for LoadImageError {}

impl From<io::Error> for LoadImageError {
    fn from(err: io::Error) -> Self {
        LoadImageError::IoError(err)
    }
}

impl From<ImageError> for LoadImageError {
    fn from(err: ImageError) -> Self {
        LoadImageError::DecodeImageError(err)
    }
}
