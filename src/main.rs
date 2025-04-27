use image::Luma;
use qrcode::{EcLevel, QrCode};
use std::env;

fn print_usage(program: &str) {
    eprintln!("Usage: {} <URL> <width> <height>", program);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    let width: u32 = args[2].parse()?;
    let height: u32 = args[3].parse()?;

    let code = QrCode::with_error_correction_level(url.as_bytes(), EcLevel::M)?;

    let image = code
        .render::<Luma<u8>>()
        .min_dimensions(width, height)
        .dark_color(Luma([0u8]))
        .light_color(Luma([255u8]))
        .build();

    image.save("qr_code.png")?;
    println!("File saved to: to qr_code.png");
    Ok(())
}
