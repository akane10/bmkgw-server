use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use bmkgw::cuaca::{self, Domain, Province};
use bmkgw::gempa::{self, Url};
use redis;
use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json::json;

mod error;

use error::Error;

fn conn_redis() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    Ok(con)
}

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
async fn get_gempa(kind: web::Path<String>) -> Result<HttpResponse, Error> {
    match Url::from_str(kind.into_inner()) {
        Some(url) => {
            let data = gempa::get_data(url).await?;
            Ok(HttpResponse::Ok().json(data))
        }
        None => Err(Error::NotFound("cannot find gempa information".to_string())),
    }
}

#[get("/cuaca/{location}")]
async fn get_cuaca(location: web::Path<String>) -> Result<HttpResponse, Error> {
    match Province::from_str(location.into_inner()) {
        Some(url) => {
            let data = cuaca::get_data(url).await;
            match data {
                Ok(val) => Ok(HttpResponse::Ok().json(val)),
                Err(e) => Err(Error::BmkgwError(e)),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Res {
    pub key: Option<String>,
}
#[get("/gempa/notif/pub_key")]
async fn get_gempa_key() -> Result<HttpResponse, Error> {
    let mut con = conn_redis()?;
    let k = con.get("public_key");

    match k {
        Ok(v) => Ok(HttpResponse::Ok().json(Res { key: Some(v) })),
        _ => Ok(HttpResponse::Ok().json(Res { key: None })),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sub {
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
}
#[post("/gempa/notif")]
async fn add_gempa_subscription(sub: web::Json<Sub>) -> Result<HttpResponse, Error> {
    let mut con = conn_redis()?;
    let auth = sub.auth.clone();
    let data: String = json!(*sub).to_string();
    con.set(auth, data)?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAuth {
    pub auth: Option<String>,
}
#[delete("/gempa/notif")]
async fn delete_gempa_subscription(sub_auth: web::Json<SubAuth>) -> Result<HttpResponse, Error> {
    match &sub_auth.auth {
        Some(v) => {
            let mut con = conn_redis()?;
            let _: () = con.del(v)?;
            Ok(HttpResponse::Ok().finish())
        }
        _ => Ok(HttpResponse::BadRequest().finish()),
    }
}

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().body("Not Found"))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(
                web::scope("/api")
                    .service(get_gempa)
                    .service(get_cuaca)
                    .service(get_locations)
                    .service(get_gempa_key)
                    .service(add_gempa_subscription)
                    .service(delete_gempa_subscription),
            )
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
