use actix_files::{Files, NamedFile};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};

mod server;
use actix_web_actors::ws;
use server::TestWebSocket;

async fn index() -> impl Responder {
    NamedFile::open_async("./dist/index.html").await.unwrap()
}

async fn test_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(TestWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to(index))
            .route("/ws", web::get().to(test_ws))
            .service(Files::new("/", "./dist"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
