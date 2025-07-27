use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, Luma};
use qrcode::{EcLevel, QrCode};
use std::env;
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;

fn print_usage(program: &str) {
    eprintln!(
        "Usage: {} <URL> <width> <height> <output.png> [--logo path/to/logo.png]",
        program
    );
}

pub fn generate_png_with_logo(
    url: &str,
    width: u32,
    height: u32,
    logo_path: Option<&str>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Generate QR code
    let code = QrCode::with_error_correction_level(url.as_bytes(), EcLevel::H)?;
    let mut qr_img = code
        .render::<Luma<u8>>()
        .min_dimensions(width, height)
        .dark_color(Luma([0u8]))
        .light_color(Luma([255u8]))
        .build();

    // Add logo if provided
    if let Some(logo_path) = logo_path {
        let logo = image::open(Path::new(logo_path))?.into_luma8();
        let logo_size = width / 5;
        let logo_resized = imageops::resize(
            &logo,
            logo_size,
            logo_size,
            image::imageops::FilterType::Lanczos3,
        );

        let x = (qr_img.width() - logo_size) / 2;
        let y = (qr_img.height() - logo_size) / 2;

        imageops::overlay(&mut qr_img, &logo_resized, x as i64, y as i64);
    }

    // Encode to PNG
    let dyn_img = DynamicImage::ImageLuma8(qr_img);
    let mut buf = Cursor::new(Vec::new());
    dyn_img.write_to(&mut buf, image::ImageFormat::Png)?;
    Ok(buf.into_inner())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    let width: u32 = args[2].parse()?;
    let height: u32 = args[3].parse()?;
    let output = &args[4];

    // Optional logo
    let mut logo_path: Option<&str> = None;
    if args.len() == 7 && args[5] == "--logo" {
        logo_path = Some(&args[6]);
    }

    let png = generate_png_with_logo(url, width, height, logo_path)?;
    let mut file = File::create(output)?;
    file.write_all(&png)?;

    println!("QR code saved to {}", output);
    Ok(())
}

