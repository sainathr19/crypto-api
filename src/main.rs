mod db;
mod helpers;
mod models;
mod routes;
mod services;
use crate::helpers::cron::start_scheduler;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use db::connection::MongoDB;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Rust Backend Server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_db: MongoDB = MongoDB::init().await.expect("Error connecting to Database");
    println!("Connected to Database");

    // Start the scheduler in a separate task
    let mongo_db_clone = mongo_db.clone();
    tokio::spawn(async move {
        if let Err(e) = start_scheduler(mongo_db_clone).await {
            eprintln!("Error starting scheduler: {}", e);
        }
    });

    let mongo_db: Data<MongoDB> = Data::new(mongo_db);

    HttpServer::new(move || {
        App::new()
            .app_data(mongo_db.clone())
            .service(home)
            .configure(routes::depths_history::init)
            .configure(routes::earnings_history::init)
            .configure(routes::swaps_history::init)
            .configure(routes::rpmuh_history::init)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
