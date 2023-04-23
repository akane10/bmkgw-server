use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};

mod error;
mod routes;

use error::Error;
use routes::cuaca::*;
use routes::gempa::*;

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello!")
}

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().body("Not Found"))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(
                web::scope("/api")
                    .service(web::scope("/cuaca").service(get_cuaca))
                    .service(web::scope("/locations").service(get_locations))
                    .service(
                        web::scope("/gempa").service(get_gempa).service(
                            web::scope("/notif")
                                .service(get_gempa_key)
                                .service(add_gempa_subscription)
                                .service(delete_gempa_subscription),
                        ),
                    ),
            )
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
