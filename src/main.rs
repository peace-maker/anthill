#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

mod schema;
mod webserver;
mod db;
//mod flag_submitter;
//mod exploit;
mod team;
//mod settings;

use clap::Parser;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Embed the sql schema into the binary.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Anthill exploit thrower
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// IP to bind webserver to
    #[clap(short, long, value_parser, default_value = "127.0.0.1")]
    address: String,

    /// Port to listen on
    #[clap(short, long, value_parser, default_value_t = 8080)]
    port: u16,

    /// Path to the frontend files
    #[clap(short, long, value_parser, default_value = "./dist")]
    frontend_path: String,
}

pub fn do_database_migration(pool: &r2d2::Pool<ConnectionManager<PgConnection>>) -> Result<(), db::Error> {
    let conn = &mut pool.get()
        .expect("Error grabbing a connection for initial migration");
    
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");
    do_database_migration(&pool).expect("Failed to migrate the database.");

    log::info!("starting HTTP server at http://{}:{}", args.address, args.port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg: &mut web::ServiceConfig| webserver::config(cfg, &args.frontend_path))
            .wrap(Logger::default())
    })
    .bind((args.address, args.port))?
    .run()
    .await
}
