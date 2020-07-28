#[macro_use]
extern crate lazy_static;

use actix_files;
use actix_web::{web, HttpServer, App, guard};
use listenfd::ListenFd;
use crate::controllers::web::home_controller::HomeController;

mod routes;
mod controllers;

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/static", "public/static").show_files_listing())
            .service(web::scope("/api/v1").guard(guard::Header("Accept", "application/json")).configure(routes::register))
            .default_service(web::route().to(HomeController::index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
// systemfd --no-pid -s http::3000 -- cargo watch -x run
// https://cheats.rs/
