use image::ImageError;
use qrcode::types::QrError;
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
