use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;

// Greet Struct (Dynamic)
#[derive(Deserialize)]
struct Info {
    name: String,
}

// Home Page Route (serving static HTML)
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/html/home.html"))
}

// About Page Route (serving static HTML)
async fn about() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/html/about.html"))
}

// Contact Page Route (serving static HTML)
async fn contact() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/html/contact.html"))
}

// Projects Page Route (serving static HTML)
async fn projects() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/html/projects.html"))
}

// Dynamic Route for Greet Page (serving dynamic HTML with embedded name)
async fn greet(info: web::Path<Info>) -> impl Responder {
    let response = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Greeting Page</title>
            <link rel="stylesheet" href="/static/css/greet.css">
        </head>
        <body>
            <div class="container">
                <h1 class="greeting">Hello, {}!</h1>
                <p>Welcome to the dynamic greeting page. Your name is displayed from the URL path.</p>
            </div>
        </body>
        </html>
        "#,
        info.name
    );
    
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
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
