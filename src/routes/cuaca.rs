use crate::Error;
use actix_web::{get, web, HttpResponse, Responder, Result};
use bmkgw::cuaca::{self, Domain, Province};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub url_param: String,
}

#[get("/cuaca/{location}")]
pub async fn get_cuaca(location: web::Path<String>) -> Result<HttpResponse, Error> {
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
pub async fn get_locations() -> Result<impl Responder> {
    let data: Vec<Location> = Domain::get_data()
        .into_iter()
        .map(|x| Location {
            name: x.name,
            url_param: x.value,
        })
        .collect();
    Ok(web::Json(data))
}
