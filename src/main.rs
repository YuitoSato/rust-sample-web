use actix_web::{App, HttpServer, web};
use listenfd::ListenFd;

use rust_sample_web::interface::todo_controller;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(todo_controller::list))
            .route("/todos", web::get().to(todo_controller::list))
            .route("/todos/{id}", web::get().to(todo_controller::find))
            .route("/todos", web::post().to(todo_controller::create))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:9000")?
    };

    server.run().await
}
