use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::{resource, get, head, ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
  cfg.service(resource("/test")
      .route(get().to(|| HttpResponse::Ok()))
      .route(head().to(|| HttpResponse::MethodNotAllowed()))
  );
}