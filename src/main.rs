use actix_files::{Files, NamedFile};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};

mod server;
use actix_web_actors::ws;
use server::TestWebSocket;
use clap::Parser;

async fn index() -> impl Responder {
    NamedFile::open_async("./dist/index.html").await.unwrap()
}

async fn test_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(TestWebSocket::new(), &req, stream)
}

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
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to(index))
            .route("/ws", web::get().to(test_ws))
            .service(Files::new("/", "./dist"))
    })
    .bind((args.address, args.port))?
    .run()
    .await
}
