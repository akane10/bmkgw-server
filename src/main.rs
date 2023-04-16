use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder, Result};
use bmkgw::cuaca::{self, Data, Domain, Province};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub url_param: String,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[get("/gempa/{kind}")]
async fn get_gempa(kind: web::Path<String>) -> impl Responder {
    format!("gempa {}!", kind)
}

#[get("/cuaca/{location}")]
async fn get_cuaca(location: web::Path<String>) -> HttpResponse {
    match Province::from_str(location.into_inner()) {
        Some(url) => {
            let data = cuaca::get_data(url).await;
            match data {
                Ok(val) => HttpResponse::Ok().json(val),
                _ => HttpResponse::NotFound().body("cannot find location"),
            }
        }
        None => HttpResponse::NotFound().body("cannot find location"),
    }
}

#[get("/locations")]
async fn get_locations() -> Result<impl Responder> {
    let data: Vec<Location> = Domain::get_data()
        .into_iter()
        .map(|x| Location {
            name: x.name,
            url_param: x.value,
        })
        .collect();
    Ok(web::Json(data))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet).service(
            web::scope("/api")
                .service(get_gempa)
                .service(get_cuaca)
                .service(get_locations),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
