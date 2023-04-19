use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder, Result};
use bmkgw::cuaca::{self, Data, Domain, Province};
use bmkgw::gempa::{self, Gempa, Url};
use serde::{Deserialize, Serialize};

mod error;

use error::Error;

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
async fn get_gempa(kind: web::Path<String>) -> Result<HttpResponse, error::Error> {
    match Url::from_str(kind.into_inner()) {
        Some(url) => {
            let data = gempa::get_data(url).await?;
            Ok(HttpResponse::Ok().json(data))
        }
        None => Err(Error::NotFound("cannot find gempa information".to_string())),
    }
}

#[get("/cuaca/{location}")]
async fn get_cuaca(location: web::Path<String>) -> Result<HttpResponse, error::Error> {
    match Province::from_str(location.into_inner()) {
        Some(url) => {
            let data = cuaca::get_data(url).await;
            match data {
                Ok(val) => Ok(HttpResponse::Ok().json(val)),
                _ => Err(Error::NotFound("cannot find location".to_string())),
            }
        }
        None => Err(Error::NotFound("cannot find location".to_string())),
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
