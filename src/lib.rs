use std::fs::Metadata;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use sqlx::{PgConnection, PgPool};
use crate::routes::{health_check, subscribe};
use log::Record;

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

pub trait Log:Sync + Send{
    fn enabled(&self, metadata: &Metadata) -> bool;
    fn log(&self, record: &Record);
    fn flush(&self);
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection pool as part of the application state
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}
