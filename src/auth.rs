use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use std::env;
use std::pin::Pin;
use futures::future::Future;

async fn validator(req: ServiceRequest, credentials: BasicAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let admin_username = env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME must be set");
    let admin_password = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");

    if credentials.user_id() == admin_username && credentials.password().map_or(false, |p| p == admin_password) {
        Ok(req)
    } else {
        Err((actix_web::error::ErrorUnauthorized("Invalid credentials"), req))
    }
}

pub fn auth_middleware() -> HttpAuthentication<BasicAuth, fn(ServiceRequest, BasicAuth) -> Pin<Box<dyn Future<Output = Result<ServiceRequest, (Error, ServiceRequest)>>>>> {
    HttpAuthentication::basic(|req, creds| Box::pin(validator(req, creds)))
}