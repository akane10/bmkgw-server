use crate::Error;
use actix_web::{delete, get, post, web, HttpResponse, Result};
use bmkgw::gempa::{self, Url};
use redis;
use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json::json;

fn conn_redis() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    Ok(con)
}

#[get("/{kind}")]
pub async fn get_gempa(kind: web::Path<String>) -> Result<HttpResponse, Error> {
    match Url::from_str(kind.into_inner()) {
        Some(url) => {
            let data = gempa::get_data(url).await?;
            Ok(HttpResponse::Ok().json(data))
        }
        None => Err(Error::NotFound("cannot find gempa information".to_string())),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Res {
    pub key: Option<String>,
}
#[get("/pub_key")]
pub async fn get_gempa_key() -> Result<HttpResponse, Error> {
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
#[post("")]
pub async fn add_gempa_subscription(sub: web::Json<Sub>) -> Result<HttpResponse, Error> {
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
#[delete("")]
pub async fn delete_gempa_subscription(
    sub_auth: web::Json<SubAuth>,
) -> Result<HttpResponse, Error> {
    match &sub_auth.auth {
        Some(v) => {
            let mut con = conn_redis()?;
            let _: () = con.del(v)?;
            Ok(HttpResponse::Ok().finish())
        }
        _ => Ok(HttpResponse::BadRequest().finish()),
    }
}
