pub mod api;
pub mod web;

mod web_view
{
    use tera::{Tera, Context};
    use actix_web::{HttpResponse, Error, error};
    use actix_web::http::StatusCode;

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

    /// Send a HTML response with specified status code.
    ///
    /// ```rust
    /// view_with_status("path_to_file.html", |context| {
    ///     context.insert("title", &"Invalid Login");
    ///     context.insert("error", &"Your username or password is wrong");
    /// }, StatusCode.UNAUTHORIZED)
    /// ```
    pub fn view_with_status<C: FnMut(&mut Context)>(path: &str, mut data: C, status: StatusCode) -> Result<HttpResponse, Error>
    {
        let mut context = Context::new();
        data(&mut context);
        // todo: redirect to 500 page on error.
        let body = TEMPLATES.render(path, &context).map_err(|e| error::ErrorInternalServerError(e))?;

        Ok(HttpResponse::build(status).content_type("text/html; charset=utf-8").body(body))
    }

    /// Send a HTML response with 200 (StatusCode::OK) status code.
    ///
    /// ```rust
    /// view("path_to_file.html", |context| {
    ///     context.insert("title", &"Invalid Login");
    ///     context.insert("error", &"Your username or password is wrong");
    /// })
    /// ```
    pub fn view<C: FnMut(&mut Context)>(path: &str, data: C) -> Result<HttpResponse, Error>
    {
        view_with_status(path, data, StatusCode::OK)
    }
}

mod api_view
{
    use actix_web::{HttpResponse, Error};
    use actix_web::http::StatusCode;

    /// Send a json response with 200 (StatusCode::OK) status code.
    ///
    /// ```rust
    /// view_with_status(r#"{
    ///     "title": "Invalid Login",
    ///     "error": "Your username or password is wrong"
    /// }"#, StatusCode.UNAUTHORIZED)
    /// ```
    pub fn view_with_status<S: Into<String>>(body: S, status: StatusCode) -> Result<HttpResponse, Error>
    {
        Ok(HttpResponse::build(status).content_type("application/json").body(body.into()))
    }

    /// Send a json response with 200 (StatusCode::OK) status code.
    ///
    /// ```rust
    /// view(r#"{
    ///     "title": "Invalid Login",
    ///     "error": "Your username or password is wrong"
    /// }"#)
    /// ```
    pub fn view<S: Into<String>>(data: S) -> Result<HttpResponse, Error>
    {
        view_with_status(data, StatusCode::OK)
    }
}

pub mod handlers
{
    use actix_web::{HttpResponse, Error, HttpRequest};
    use actix_web::http::{header::ACCEPT, StatusCode};

    pub async fn page_not_found(req: HttpRequest) -> Result<HttpResponse, Error>
    {
        match req.headers().get(ACCEPT) {
            Some(option) if option.eq("application/json")  => {
                super::api_view::view_with_status( "{\"error\": \"content not found\"}", StatusCode::NOT_FOUND)
            } _ => {
                super::web_view::view_with_status("404.html", |_| {}, StatusCode::NOT_FOUND)
            }
        }
    }

    //@todo implement 500 handler
}
