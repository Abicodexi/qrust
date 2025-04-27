use image::{ImageBuffer, ImageError, Luma};
use qrcode::types::QrError;
use qrcode::{EcLevel, QrCode};
use std::fmt;

/// Error type for QR-code generation failures.
#[derive(Debug)]
pub enum GenerateError {
    /// Underlying QR-code library error
    Qr(QrError),
    /// PNG rendering error
    Image(ImageError),
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateError::Qr(e) => write!(f, "QR generation error: {}", e),
            GenerateError::Image(e) => write!(f, "PNG rendering error: {}", e),
        }
    }
}

impl std::error::Error for GenerateError {}

impl From<QrError> for GenerateError {
    fn from(e: QrError) -> Self {
        GenerateError::Qr(e)
    }
}

impl From<ImageError> for GenerateError {
    fn from(e: ImageError) -> Self {
        GenerateError::Image(e)
    }
}

pub fn generate_png(url: &str, width: u32, height: u32) -> Result<Vec<u8>, GenerateError> {
    let code = QrCode::with_error_correction_level(url.as_bytes(), EcLevel::M)?;

    let image: ImageBuffer<Luma<u8>, _> = code
        .render::<Luma<u8>>()
        .min_dimensions(width, height)
        .dark_color(Luma([0u8]))
        .light_color(Luma([255u8]))
        .build();

    let dyn_image = image::DynamicImage::ImageLuma8(image);
    let mut buffer = std::io::Cursor::new(Vec::new());
    dyn_image.write_to(&mut buffer, image::ImageFormat::Png)?;
    Ok(buffer.into_inner())
}
