use actix_web::Responder;
use crate::controllers::web_view::view;

pub struct HomeController;

impl HomeController
{
    pub async fn index() -> impl Responder
    {
        view("index.html", | context| {
            context.insert("title", &"Demo App");
            context.insert("header", &"Demo App");
            context.insert("content", &"Some random Demo");
        })
    }
}
