use actix_web::{error, Responder, HttpResponse, Result, Error};
use tera::{Tera, Context};

pub mod api;
pub mod web;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("public/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

//@todo: allow status


pub fn web_view<C: FnMut(&mut Context)>(path: &str, mut data: C) -> Result<HttpResponse, Error>
{
    let mut context = Context::new();
    data(&mut context);

    // todo: redirect to 500 page on error.
    let content = TEMPLATES.render(path, &context).map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML_UTF_8.to_string()).body(content))
}

//@todo make to accept both string and struct
pub fn api_view<S: Into<String>>(data: S) -> impl Responder
{
    HttpResponse::Ok().content_type(mime::APPLICATION_JSON.to_string()).body(data.into())
}
