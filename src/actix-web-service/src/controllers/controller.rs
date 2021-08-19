use actix_web::web::{ServiceConfig};

pub trait Controller {
  fn configure_routes(cfg: &mut ServiceConfig);
}