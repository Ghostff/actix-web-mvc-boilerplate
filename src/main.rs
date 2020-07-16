#[macro_use]
extern crate lazy_static;

use actix_web::{web, HttpServer, App, guard, HttpRequest, HttpResponse};
use actix_web::http::header::ACCEPT;
use listenfd::ListenFd;

mod routes;
mod controllers;

/// 404 handler
/// //@todo use the view method instead.
async fn p404(req: HttpRequest) -> HttpResponse
{
    let mut content_type = mime::TEXT_HTML_UTF_8.to_string();

    let body = match req.headers().get(ACCEPT) {
        Some(option) if option.eq("application/json")  => {
            content_type = String::from("application/json");
            "{\"error\": \"content not found\"}"
        } _ => {
            include_str!("../public/404.html")
        }
    };

    HttpResponse::NotFound().content_type(content_type).body(body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {

        let is_json = guard::Header("Accept", "application/json");
        use {routes::web::register_web as web_route, routes::api::register_api as api_route};

        App::new()
            .configure(web_route)
            .service(web::scope("/api/v1").guard(is_json).configure(api_route))
            .default_service(web::route().to(p404))
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
