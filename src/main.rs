mod config;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use config::AppConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

async fn postgres_post(task: web::Json<Task>) -> impl Responder {
    println!("{:?}", task.into_inner());
    HttpResponse::Created()
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

    println!("{:?}", config.database.connection_string());

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(
                web::scope("/v1")
                    .service(web::scope("/data").route("", web::get().to(data)))
                    .service(
                        web::scope("/postgres").service(
                            web::resource("")
                                .route(web::get().to(postgres))
                                .route(web::post().to(postgres_post)),
                        ),
                    )
                    .service(
                        web::scope("/search")
                            .service(web::resource("").route(web::get().to(search))),
                    ),
            )
            .route("/", web::get().to(index))
    })
    .bind(config.addr())?
    .run()
    .await
}
