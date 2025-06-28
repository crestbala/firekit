mod services;

use std::net::SocketAddr;
use std::str::FromStr;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mime_guess::from_path;
use rust_embed::Embed;
use services::pokemon;

#[derive(Embed)]
#[folder = "../client/build/"]
struct Assets;

async fn serve_frontend(path: web::Path<String>) -> impl Responder {
    let file_path = if path.is_empty() {
        "index.html"
    } else {
        path.as_str()
    };

    // Serve static assets
    if let Some(content) = Assets::get(file_path) {
        return HttpResponse::Ok()
            .content_type(from_path(file_path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned());
    }

    // If no extension, try {path}/index.html to serve pre-rendered HTML files
    if !file_path.contains('.') && !file_path.is_empty() {
        let index_path = format!("{}/index.html", file_path);
        if let Some(content) = Assets::get(&index_path) {
            return HttpResponse::Ok()
                .content_type("text/html")
                .body(content.data.into_owned());
        }

        // If no prerendered file, fall back to root index.html (SvelteKit SPA routing)
        if let Some(content) = Assets::get("index.html") {
            return HttpResponse::Ok()
                .content_type("text/html")
                .body(content.data.into_owned());
        }
    }

    HttpResponse::NotFound().body("404 Not Found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let is_dev = cfg!(debug_assertions);

    let addr_str = if is_dev {
        "127.0.0.1:8080" // For local development
    } else {
        "0.0.0.0:8080" // For production
    };
    let addr = SocketAddr::from_str(addr_str).expect("Failed to parse address");

    println!("🦀 {} mode: server listening on {}", if is_dev {"DEV"} else {"PROD"}, addr);

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .wrap(Cors::permissive())
                    .route("/search", web::get().to(pokemon::search))
                    .route("/random", web::get().to(pokemon::get_n_random)),
            )
            .route("/{path:.*}", web::get().to(serve_frontend))
    })
    .bind(addr)?
    .run()
    .await
}
