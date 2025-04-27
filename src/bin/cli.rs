use image::{ImageBuffer, Luma};
use qr_generator::GenerateError;
use qrcode::{EcLevel, QrCode};
use std::env;
use std::fs::File;
use std::io::Write;

fn print_usage(program: &str) {
    eprintln!("Usage: {} <URL> <width> <height> <output.png>", program);
}

pub fn generate_png(url: &str, width: u32, height: u32) -> Result<Vec<u8>, GenerateError> {
    let code = QrCode::with_error_correction_level(url.as_bytes(), EcLevel::M)?;

    let image: ImageBuffer<Luma<u8>, _> = code
        .render::<Luma<u8>>()
        .min_dimensions(width, height)
        .dark_color(Luma([0u8]))
        .light_color(Luma([255u8]))
        .build();
    let dyn_img = image::DynamicImage::ImageLuma8(image);
    let mut buf = std::io::Cursor::new(Vec::new());
    dyn_img.write_to(&mut buf, image::ImageFormat::Png)?;
    Ok(buf.into_inner())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    let width: u32 = args[2].parse()?;
    let height: u32 = args[3].parse()?;
    let output = &args[4];

    let png = generate_png(url, width, height)?;
    let mut file = File::create(output)?;
    file.write_all(&png)?;
    println!("File saved to: {}", output);
    Ok(())
}
