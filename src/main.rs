#[macro_use]
extern crate lazy_static;

use actix_web::{web, HttpServer, App, guard};
use listenfd::ListenFd;
use {routes::web::register_web as web_route, routes::api::register_api as api_route};

mod routes;
mod controllers;

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(web_route)
            .service(web::scope("/api/v1").guard(guard::Header("Accept", "application/json")).configure(api_route))
            .default_service(web::route().to(crate::controllers::handlers::page_not_found))
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
