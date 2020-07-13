use actix_web::Scope;
use actix_web::web::{resource, get};

use crate::controllers::api::home_controller:: {
    HomeController
};

pub fn register_api(scope : Scope) -> Scope
{
    scope.service(resource("/").route(get().to(HomeController::index)))
}
