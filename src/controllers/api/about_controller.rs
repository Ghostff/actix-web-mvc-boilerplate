use actix_web::Responder;
use crate::controllers::api_view::view;

pub struct AboutController;

impl AboutController
{
    pub async fn index() -> impl Responder
    {
        view( r#"{
            "name": "Foo Bar",
            "address":  {
                "city": "foo",
                "state": "bar",
                "country": "foobar"
            },
            "email": "foo@bar.com"
        }"#)
    }

/*    pub fn name(mut foo : String) {

    }*/
}


