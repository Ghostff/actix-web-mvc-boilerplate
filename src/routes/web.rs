use actix_web::web::{resource, get, ServiceConfig};

use crate::controllers::web:: {
    home_controller::HomeController,
    about_controller::AboutController
};

pub fn register_web(config: &mut ServiceConfig)
{
    config.service(resource("/").route(get().to(HomeController::index)));
    config.service(resource("/about").route(get().to(AboutController::index)));
}
