use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[get("/gempa/{kind}")]
async fn get_gempa(kind: web::Path<String>) -> impl Responder {
    format!("gempa {}!", kind)
}

#[get("/cuaca/{location}")]
async fn get_cuaca(location: web::Path<String>) -> impl Responder {
    format!("cuaca {}!", location)
}

#[get("/locations")]
async fn get_locations() -> impl Responder {
    format!("locations!")
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
