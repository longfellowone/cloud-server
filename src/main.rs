mod configuration;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use configuration::Configuration;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use std::fmt::Debug;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Configuration::new()?;

    println!("{:?}", std::env::var("DATABASE_URL").unwrap());

    let db = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(10))
        .connect_with(config.postgres.options())
        .await?;

    let db = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(db.clone())
            .configure(routes)
    })
    .bind(config.server.addr())?
    .run()
    .await?;

    Ok(())
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(web::scope("/data").route("", web::get().to(data)))
            .service(
                web::scope("/postgres")
                    .service(
                        web::resource("")
                            .route(web::get().to(postgres::list))
                            .route(web::post().to(postgres::post)),
                    )
                    .service(web::resource("/{id}").route(web::get().to(postgres::get))),
            )
            .service(web::scope("/search").service(web::resource("").route(web::get().to(search)))),
    );
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    id: i32,
    name: String,
}

async fn data() -> impl Responder {
    HttpResponse::Ok().json(Task {
        id: 1,
        name: "data".to_string(),
    })
}

mod postgres {
    use super::Task;
    use actix_web::{web, HttpResponse, Responder};
    use sqlx::PgPool;

    pub async fn list(db: web::Data<PgPool>) -> impl Responder {
        let result = sqlx::query_as!(Task, "SELECT id, name FROM tasks")
            .fetch_all(db.as_ref())
            .await;

        println!("list hit");

        match result {
            Ok(tasks) => HttpResponse::Ok().json(tasks),
            Err(err) => HttpResponse::NotFound().body(err.to_string()),
        }
    }

    pub async fn get(id: web::Path<u32>) -> impl Responder {
        println!("got id: {:?}", id.into_inner());

        HttpResponse::Ok().json(Task {
            id: 2,
            name: "get postgres data".to_string(),
        })
    }

    pub async fn post(task: web::Form<Task>) -> impl Responder {
        let task = task.into_inner();

        println!("got task: {:?}", task);

        HttpResponse::Created().json(task)
    }
}

async fn search() -> impl Responder {
    HttpResponse::Ok().json(Task {
        id: 5,
        name: "search data".to_string(),
    })
}
