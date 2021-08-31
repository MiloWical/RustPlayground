use crate::controllers::controller::Controller;
use crate::handlers::shopping_cart_handler::ShoppingCartHandler;
use crate::shopping_cart::in_memory_cart_db::InMemoryCartDb;
use actix_web::{get, App, HttpServer, Result};
use actix_web::web::{get, ServiceConfig, Path, Data};

pub struct ShoppingCartController;

impl Controller for ShoppingCartController {
  fn configure_routes(cfg: &mut ServiceConfig) {
    cfg
      .data(ShoppingCartHandler::new(
          Box::new(InMemoryCartDb::new(""))
        )) 
      .service(get_cart);
  }
}

#[get("/cart/{id}")] // <- define path parameters
pub async fn get_cart(data: Data<ShoppingCartHandler>, id: Path<String>) -> Result<String> {  
  Ok(format!("Card ID: {}", id))
}