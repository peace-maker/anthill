use actix_files::{NamedFile, Files};
use actix_web::{Responder, HttpServer, App, web};



async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to(index))
            .service(Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
