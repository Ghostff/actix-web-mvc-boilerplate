#[macro_use]
extern crate lazy_static;

use actix_web::{web, HttpServer, App};
use listenfd::ListenFd;

mod routes;
mod controllers;

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(move |cfg: &mut web::ServiceConfig| {
        routes::web::register_web(cfg);
        cfg.service(routes::api::register_api(web::scope("/api")));
    }));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}

// systemfd --no-pid -s http::3000 -- cargo watch -x run
// https://cheats.rs/
