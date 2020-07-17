use actix_web::Responder;
use crate::controllers::web_view::view;

pub struct AboutController;

impl AboutController
{
    pub async fn index() -> impl Responder
    {
        view("about.html", | context| {
            context.insert("about_content", &"Some random about page content");
        })
    }
}
