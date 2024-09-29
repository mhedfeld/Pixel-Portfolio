use actix_files::Files; 
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;

// Greet Struct ( Dynamic)
#[derive(Deserialize)]
struct Info {
    name: String,
}

// Home Page Route (serving static HTML)
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/assets/html/home.html"))
}

// About Page Route (serving static HTML)
async fn about() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/assets/html/about.html"))
}

// Contact Page Route (serving static HTML)
async fn contact() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/assets/html/contact.html"))
}

// Projects Page Route (serving static HTML)
async fn projects() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/assets/html/projects.html"))
}

// Dynamic Route for Greet Page
async fn greet(info: web::Path<Info>) -> impl Responder {
    let response = format!("Hello, {}! This is my dynamic greeting page!", info.name);
    HttpResponse::Ok().body(response)
}

// Main function setting up the Actix Web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Static routes serving HTML pages
            .route("/", web::get().to(home))        // Home Page Route
            .route("/about", web::get().to(about))  // About Page Route
            .route("/contact", web::get().to(contact)) // Contact Page Route
            .route("/projects", web::get().to(projects)) // Projects Page Route
            .route("/greet/{name}", web::get().to(greet)) // Dynamic Greet Route
            
            // Serve static files (CSS, JS, images, fonts) from the assets directory
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
