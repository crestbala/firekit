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

async fn serve_frontend(req: HttpRequest) -> impl Responder {
    serve_frontend_with_fallback(req, "index.html").await
}

async fn serve_frontend_with_fallback(req: HttpRequest, fallback_file: &str) -> impl Responder {
    let requested_path = req.path().to_string();
    println!("🔍 Raw request path: '{}'", requested_path);

    let cleaned = requested_path.trim_start_matches('/');
    println!("🔍 Cleaned path before processing: '{}'", cleaned);

    let serve_path = if cleaned.is_empty() {
        fallback_file.to_string()
    } else {
        cleaned.to_string()
    };

    let base_dir = "../client/build";
    let fallback_file_owned = fallback_file.to_string();

    // Clone what we need for the blocking operation
    let serve_path_clone = serve_path.clone();
    let base_dir_clone = base_dir.to_string();
    let fallback_file_clone = fallback_file_owned.clone();

    let result = web::block(move || -> Result<(Vec<u8>, bool), std::io::Error> {
        let specific_path = format!("{}/{}", base_dir_clone, serve_path_clone);

        println!("📂 Full path attempting to read: '{}'", specific_path);

        // Check if it's a file path or a route
        if std::path::Path::new(&specific_path).exists() {
            println!("✓ Path exists, attempting to read");
            match std::fs::read(&specific_path) {
                Ok(bytes) => {
                    println!("✓ Successfully read {} bytes", bytes.len());
                    return Ok((bytes, false)); // false = not fallback
                }
                Err(e) => {
                    println!("✗ Failed to read: {}", e);
                }
            }
        } else {
            println!("✗ Path does not exist: {}", specific_path);
        }

        // Fall back to the specified HTML file
        let fallback_path = format!("{}/{}", base_dir_clone, fallback_file_clone);
        println!("🔄 Trying fallback: {}", fallback_path);

        match std::fs::read(&fallback_path) {
            Ok(bytes) => {
                println!("✓ Fallback successful: {} bytes", bytes.len());
                Ok((bytes, true)) // true = is fallback
            }
            Err(e) => {
                println!("✗ Fallback failed: {}", e);
                Err(e)
            }
        }
    })
    .await;

    match result {
        Ok((bytes, is_fallback)) => {
            // If we're serving the fallback HTML, always use text/html
            let content_type = if is_fallback {
                "text/html; charset=utf-8"
            } else {
                get_content_type(&serve_path)
            };
            println!(
                "→ Serving {} bytes with content-type: {}\n",
                bytes.len(),
                content_type
            );
            HttpResponse::Ok().content_type(content_type).body(bytes)
        }
        Err(e) => {
            println!("→ Returning 404: {}\n", e);
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
