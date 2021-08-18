//use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::{resource, get, head, ServiceConfig};

use crate::hello_world::routes::*;

pub fn routes(cfg: &mut ServiceConfig) {
  cfg
    // .service(resource("/test")
    // .route(get().to(|| HttpResponse::Ok()))
    // .route(head().to(|| HttpResponse::MethodNotAllowed())));
    .service(hello)
    .service(echo)
    .route("/hey", get().to(manual_hello));
}