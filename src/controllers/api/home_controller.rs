use actix_web::Responder;
use crate::controllers::api_view as view;

pub struct HomeController;

impl HomeController
{
    pub async fn index() -> impl Responder
    {
        view( r#"{
            "title": "Demo App",
            "header": "Demo App",
            "content": "Some random Demo"
        }"#)
    }
}


