use qr_generator::generate_png;
use std::env;
use std::fs::File;
use std::io::Write;

fn print_usage(program: &str) {
    eprintln!("Usage: {} <URL> <width> <height> <output.png>", program);
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
