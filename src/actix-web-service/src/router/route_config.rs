//use actix_web::{get, post, HttpResponse, Responder};
use actix_web::{Route};
use actix_web::web::{resource, get, head, ServiceConfig};

use crate::hello_world::*;

// TODO: Figure this out!
// pub struct Router {
//   routes: Vec<(String, Route)>
// }

// impl Router {
//   fn new() -> Self {
//     routes: <Vec<(String, Route)>>::new
//   }

//   pub fn routes(&self) -> Vec<(String, Route)> {
//     self.routes
//   }

//   pub fn add(&self, new_route: &(String, Route)) {
//     &self.routes.add(new_route);
//   }

//   pub fn configure(cfg: &mut ServiceConfig) {

//   }
// }

pub fn routes(cfg: &mut ServiceConfig) {
  cfg
    // .service(resource("/test")
    // .route(get().to(|| HttpResponse::Ok()))
    // .route(head().to(|| HttpResponse::MethodNotAllowed())));
    .service(hello)
    .service(echo)
    //.service()
    .route("/hey", get().to(manual_hello));
}