mod config;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use config::AppConfig;
use serde::Serialize;

#[derive(Serialize)]
struct Task {
    id: i32,
    name: String,
}

async fn index() -> impl Responder {
    "index"
}

async fn data() -> impl Responder {
    web::Json(Task {
        id: 1,
        name: "fixed data".to_string(),
    })
}

async fn postgres() -> impl Responder {
    web::Json(Task {
        id: 2,
        name: "postgres data".to_string(),
    })
}

async fn search() -> impl Responder {
    web::Json(Task {
        id: 3,
        name: "search data".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::new().unwrap();

    println!("{:?}", config.port);

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/", web::get().to(index))
            .route("/data", web::get().to(data))
            .route("/postgres", web::get().to(postgres))
            .route("/search", web::get().to(search))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
