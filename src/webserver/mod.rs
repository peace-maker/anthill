use actix_files::{Files, NamedFile};
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};

use actix_web_actors::ws;
use crate::DbPool;
mod websocket;
mod rest_api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
    .configure(rest_api::config)
    .route("/ws", web::get().to(handle_websocket))
    .service(Files::new("/", "./dist"));
}

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./dist/index.html").await.unwrap()
}

async fn handle_websocket(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    ws::start(websocket::WsApiSession::new(pool.get_ref().clone()), &req, stream)
}
