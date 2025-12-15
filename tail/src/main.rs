mod services;
use ntex::http::{header, Method};
use ntex::web::{self, App, HttpRequest, HttpResponse, HttpServer, Responder};
use ntex_cors::Cors;
use services::pokemon;
use std::net::SocketAddr;
use std::path::Path as StdPath;
use std::str::FromStr;

fn get_content_type(path: &str) -> &str {
    match StdPath::new(path).extension().and_then(|e| e.to_str()) {
        Some("html") | Some("htm") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js" | "mjs") => "application/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn has_file_extension(path: &str) -> bool {
    StdPath::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .is_some()
}

async fn serve_frontend(req: HttpRequest) -> impl Responder {
    let requested_path = req.path().to_string();
    println!("🔍 Raw request path: '{}'", requested_path);

    let cleaned = requested_path.trim_start_matches('/');

    // Check if this looks like a static asset request
    let is_static_asset = cleaned.starts_with("_app/")
        || cleaned.starts_with("assets/")
        || cleaned.starts_with("static/")
        || has_file_extension(cleaned);

    let base_dir = "../front/build";

    if is_static_asset {
        // Try to serve the actual file
        let file_path = format!("{}/{}", base_dir, cleaned);
        println!("📂 Attempting to serve static file: '{}'", file_path);

        let file_path_clone = file_path.clone();
        let cleaned_clone = cleaned.to_string();

        let result = web::block(move || -> Result<Vec<u8>, std::io::Error> {
            std::fs::read(&file_path_clone)
        })
        .await;

        match result {
            Ok(bytes) => {
                let content_type = get_content_type(&cleaned_clone);
                println!("✓ Served static file: {} bytes\n", bytes.len());
                return HttpResponse::Ok().content_type(content_type).body(bytes);
            }
            Err(e) => {
                println!("✗ Static file not found: {}\n", e);
                return HttpResponse::NotFound()
                    .content_type("text/plain")
                    .body("File not found");
            }
        }
    }

    // For non-static paths, serve index.html (SPA fallback)
    println!("🔄 Serving SPA fallback (index.html)");

    let fallback_path = format!("{}/index.html", base_dir);
    let result =
        web::block(move || -> Result<Vec<u8>, std::io::Error> { std::fs::read(&fallback_path) })
            .await;

    match result {
        Ok(bytes) => {
            println!("✓ Served index.html: {} bytes\n", bytes.len());
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(bytes)
        }
        Err(e) => {
            println!("✗ index.html not found: {}\n", e);
            HttpResponse::NotFound()
                .content_type("text/html; charset=utf-8")
                .body("<h1>404 - Page Not Found</h1>")
        }
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let is_dev = cfg!(debug_assertions);
    let addr_str = if is_dev {
        "127.0.0.1:8080"
    } else {
        "0.0.0.0:8080"
    };
    let addr = SocketAddr::from_str(addr_str).expect("Failed to parse address");
    println!(
        "🦀 {} mode: server listening on {}",
        if is_dev { "DEV" } else { "PROD" },
        addr
    );

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .wrap(
                        Cors::new()
                            .allowed_origin("http://localhost:5173")
                            .allowed_methods(vec![Method::GET, Method::POST])
                            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                            .allowed_header(header::CONTENT_TYPE)
                            .max_age(3600)
                            .finish(),
                    )
                    .route("/search", web::get().to(pokemon::search))
                    .route("/random", web::get().to(pokemon::get_n_random)),
            )
            .default_service(web::route().to(serve_frontend))
    })
    .bind(addr)?
    .run()
    .await
}
