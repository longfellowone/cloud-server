mod config;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use config::AppConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
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

mod postgres {
    use super::Task;
    use actix_web::{web, HttpResponse, Responder};

    pub async fn list() -> impl Responder {
        let task1 = Task {
            id: 3,
            name: "list postgres data".to_string(),
        };
        let task2 = Task {
            id: 4,
            name: "list postgres data".to_string(),
        };

        let tasks = vec![task1, task2];

        web::Json(tasks)
    }

    pub async fn get(task: web::Path<u32>) -> impl Responder {
        println!("got task_id: {:?}", task);
        web::Json(Task {
            id: 2,
            name: "get postgres data".to_string(),
        })
    }

    pub async fn post(task: web::Json<Task>) -> impl Responder {
        println!("got task: {:?}", task.into_inner());
        HttpResponse::Created()
    }
}

async fn search() -> impl Responder {
    web::Json(Task {
        id: 5,
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
                        web::scope("/postgres")
                            .service(
                                web::resource("")
                                    .route(web::get().to(postgres::list))
                                    .route(web::post().to(postgres::post)),
                            )
                            .service(
                                web::resource("/{task_id}").route(web::get().to(postgres::get)),
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
