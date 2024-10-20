use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result as SqliteResult};
use tera::Tera;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use lazy_static::lazy_static;
use actix_web::error::ResponseError;
use std::fmt;

mod auth; // Ensure this is at the top of your main.rs
use crate::auth::auth_middleware; // This should work if auth.rs is 

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

struct AppState {
    db_pool: Pool<SqliteConnectionManager>,
    templates: Tera,
}

// Custom error type to handle rusqlite errors
#[derive(Debug)]
enum CustomError {
    RusqliteError(rusqlite::Error),
    ActixError(actix_web::Error),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::RusqliteError(e) => write!(f, "Rusqlite error: {}", e),
            CustomError::ActixError(e) => write!(f, "Actix error: {}", e),
        }
    }
}

impl ResponseError for CustomError {}

impl From<rusqlite::Error> for CustomError {
    fn from(error: rusqlite::Error) -> Self {
        CustomError::RusqliteError(error)
    }
}

impl From<actix_web::Error> for CustomError {
    fn from(error: actix_web::Error) -> Self {
        CustomError::ActixError(error)
    }
}

//Project Struct 
#[derive(Serialize, Deserialize)]
struct Project {
    id: i32,
    title: String,
    description: String,
    link: String,
    image: String,
}

//Contact Submission Struct
#[derive(Serialize)]
struct Contact {
    id: i32,
    name: String,
    email: String,
    phone: Option<String>,
    message: String,
}

// Greet Struct 
#[derive(Deserialize)]
struct Info {
    name: String,
}

//Contact Form Struct 
#[derive(Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    phone: Option<String>,
    message: String,
}

//Delete Form Struct 
#[derive(Deserialize)]
struct DeleteForm {
    id: i32,
}

//Admin Page Route
async fn admin_messages(data: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    let conn = data.db_pool.get().map_err(|e| {
        eprintln!("Failed to get db connection: {}", e);
        CustomError::ActixError(actix_web::error::ErrorInternalServerError("Database error"))
    })?;

    let mut stmt = conn.prepare("SELECT id, name, email, phone, message FROM contacts")?;
    let contacts_iter = stmt.query_map([], |row| {
        Ok(Contact {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            phone: row.get(3)?,
            message: row.get(4)?,
        })
    })?;

    let contacts: Vec<Contact> = contacts_iter.collect::<SqliteResult<_>>()?;

    let mut context = tera::Context::new();
    context.insert("contacts", &contacts);

    let rendered = data.templates.render("admin.html", &context)
        .map_err(|e| {
            eprintln!("Template rendering error: {}", e);
            CustomError::ActixError(actix_web::error::ErrorInternalServerError("Template error"))
        })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn add_project(form: web::Form<Project>, data: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    let conn = data.db_pool.get().map_err(|e| {
        eprintln!("Failed to get db connection: {}", e);
        CustomError::ActixError(actix_web::error::ErrorInternalServerError("Database error"))
    })?;

    conn.execute(
        "INSERT INTO projects (title, description, link, image) VALUES (?1, ?2, ?3, ?4)",
        params![form.title, form.description, form.link, form.image],
    )?;

    Ok(HttpResponse::SeeOther().append_header(("Location", "/admin")).finish())
}
async fn project_detail(data: web::Data<AppState>, project_id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let conn = data.db_pool.get().map_err(|e| {
        eprintln!("Failed to get db connection: {}", e);
        CustomError::ActixError(actix_web::error::ErrorInternalServerError("Database error"))
    })?;

    let project = conn.query_row(
        "SELECT id, title, description, link, image FROM projects WHERE id = ?1",
        params![project_id.into_inner()],
        |row| {
            Ok(Project {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                link: row.get(3)?,
                image: row.get(4)?,
            })
        },
    ).map_err(|e| {
        eprintln!("Failed to fetch project: {}", e);
        CustomError::ActixError(actix_web::error::ErrorNotFound("Project not found"))
    })?;

    let mut context = tera::Context::new();
    context.insert("project", &project);

    let rendered = data.templates.render("project_detail.html", &context)
        .map_err(|e| {
            eprintln!("Template rendering error: {}", e);
            CustomError::ActixError(actix_web::error::ErrorInternalServerError("Template error"))
        })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn delete_contact(form: web::Form<DeleteForm>, data: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    let conn = data.db_pool.get().map_err(|e| {
        eprintln!("Failed to get db connection: {}", e);
        CustomError::ActixError(actix_web::error::ErrorInternalServerError("Database error"))
    })?;

    conn.execute("DELETE FROM contacts WHERE id = ?1", params![form.id])?;

    Ok(HttpResponse::SeeOther().append_header(("Location", "/admin")).finish())
}

// Home Page Route 
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/html/home.html"))
}

// About Page Route 
async fn about() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/html/about.html"))
}

// Contact Page Route 
async fn contact() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/html/contact.html"))
}

//Projects Page Route 
async fn projects(data: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    let context = tera::Context::new();
    let rendered = data.templates.render("projects.html", &context)
        .map_err(|e| {
            eprintln!("Template rendering error: {}", e);
            CustomError::ActixError(actix_web::error::ErrorInternalServerError("Template error"))
        })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}
async fn projects_data() -> Result<HttpResponse, CustomError> {
    let conn = Connection::open("pixel_database.db").map_err(|e| {
        eprintln!("Failed to open database: {}", e);
        CustomError::ActixError(actix_web::error::ErrorInternalServerError("Database error"))
    })?;

    let mut stmt = conn.prepare("SELECT id, title, description, link, image FROM projects").map_err(|e| {
        eprintln!("Failed to prepare statement: {}", e);
        CustomError::ActixError(actix_web::error::ErrorInternalServerError("Database error"))
    })?;

    let project_iter = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            link: row.get(3)?,
            image: row.get(4)?,
        })
    }).map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        CustomError::ActixError(actix_web::error::ErrorInternalServerError("Database error"))
    })?;

    let projects: Vec<Project> = project_iter.collect::<Result<_, _>>().map_err(|e| {
        eprintln!("Failed to collect projects: {}", e);
        CustomError::ActixError(actix_web::error::ErrorInternalServerError("Database error"))
    })?;

    Ok(HttpResponse::Ok().json(projects))
}

// Dynamic Route for Greet Page 
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

//Handle Contact Form Submissions
async fn submit_contact(form: web::Form<ContactForm>) -> Result<HttpResponse, CustomError> {
    let conn = Connection::open("pixel_database.db").unwrap();
    conn.execute( 
        "INSERT INTO contacts (name, email, phone, message) VALUES (?1, ?2, ?3, ?4)",
        params![form.name, form.email, form.phone, form.message],
    )?;

    Ok(HttpResponse::Ok().body("Form Submitted Successfully!"))
}

// Main function / Actix Web Server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = SqliteConnectionManager::file(database_url);
    let pool = Pool::<SqliteConnectionManager>::new(manager).expect("Failed to create pool.");
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db_pool: pool.clone(), templates: tera.clone() }))
            .route("/", web::get().to(home))
            .route("/about", web::get().to(about))
            .route("/contact", web::get().to(contact))
            .route("/projects", web::get().to(projects))
            .route("/greet/{name}", web::get().to(greet))
            .route("/submit_contact", web::post().to(submit_contact))
            .route("/projects_data", web::get().to(projects_data))
            .route("/project/{id}", web::get().to(project_detail))
                .service(
                web::scope("/admin")
                    .wrap(auth_middleware())
                    .route("", web::get().to(admin_messages))
                    .route("/add_project", web::post().to(add_project))
                    .route("/delete_contact", web::post().to(delete_contact))
            )
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}