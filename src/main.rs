use actix_cors::Cors;
use actix_web::dev::ServiceRequest;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::basic::Config;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use env_logger::Env;

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

async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let user_id = dotenv::var("USER_ID").expect("missing USER_ID");
    let pass = dotenv::var("PASSWORD").expect("missing PASSWORD");
    let config = req.app_data::<Config>().cloned().unwrap_or_default();

    if credentials.user_id().eq(&user_id) && credentials.password().unwrap().trim().eq(&pass) {
        Ok(req)
    } else {
        Err((AuthenticationError::from(config).into(), req))
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let port = match dotenv::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_) => 8000,
    };

    println!("port {}", port);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let basic_auth = HttpAuthentication::basic(validator);
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(index)
            .service(
                web::scope("/api")
                    .service(web::scope("/cuaca").service(get_cuaca))
                    .service(web::scope("/locations").service(get_locations))
                    .service(
                        web::scope("/gempa").service(get_gempa).service(
                            web::scope("/notif")
                                .wrap(basic_auth)
                                .service(get_gempa_key)
                                .service(add_gempa_subscription)
                                .service(delete_gempa_subscription),
                        ),
                    ),
            )
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
