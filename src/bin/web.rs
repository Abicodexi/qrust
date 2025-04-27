use actix_web::{web, App, HttpResponse, HttpServer, Result};
use qr_generator::generate_png;
use serde::Deserialize;

#[derive(Deserialize)]
struct QRParams {
    url: String,
    width: Option<u32>,
    height: Option<u32>,
}

async fn generate_qr(query: web::Query<QRParams>) -> Result<HttpResponse> {
    let width = query.width.unwrap_or(300);
    let height = query.height.unwrap_or(300);
    let png = generate_png(&query.url, width, height)
        .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

    Ok(HttpResponse::Ok().content_type("image/png").body(png))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080");
    HttpServer::new(|| App::new().route("/", web::get().to(generate_qr)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
