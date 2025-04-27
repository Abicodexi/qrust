use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct QRParams {
    url: String,
    width: Option<u32>,
    height: Option<u32>,
    logo_url: Option<String>,
}

async fn generate_qr(query: web::Query<QRParams>) -> Result<HttpResponse> {
    let width = query.width.unwrap_or(300);
    let height = query.height.unwrap_or(300);

    let mut qr_img = {
        let code =
            qrcode::QrCode::with_error_correction_level(query.url.as_bytes(), qrcode::EcLevel::H)
                .map_err(|e| actix_web::error::ErrorBadRequest(e))?;
        code.render::<image::Luma<u8>>()
            .min_dimensions(width, height)
            .build()
    };

    if let Some(ref logo_url) = query.logo_url {
        let resp = reqwest::get(logo_url)
            .await
            .map_err(|e| actix_web::error::ErrorBadRequest(e))?;
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

        let logo =
            image::load_from_memory(&bytes).map_err(|e| actix_web::error::ErrorBadRequest(e))?;

        let logo_size = width / 5;
        let logo = logo
            .resize_exact(logo_size, logo_size, image::imageops::Lanczos3)
            .into_luma8();

        let x = (qr_img.width() - logo_size) / 2;
        let y = (qr_img.height() - logo_size) / 2;
        image::imageops::overlay(&mut qr_img, &logo, x as i64, y as i64);
    }

    // 3) Encode final image to PNG bytes
    let dyn_img = image::DynamicImage::ImageLuma8(qr_img);
    let mut buf = std::io::Cursor::new(Vec::new());
    dyn_img
        .write_to(&mut buf, image::ImageFormat::Png)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(buf.into_inner()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080");
    HttpServer::new(|| App::new().route("/", web::get().to(generate_qr)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
