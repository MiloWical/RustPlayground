use crate::controllers::controller::Controller;
use crate::shopping_cart::cart_db::CartDb;
use actix_web::{App, HttpServer, Result};
use actix_web::web::{get, ServiceConfig, Path, Data};

pub struct ShoppingCartHandler {
  db: Box<dyn CartDb>
}

impl ShoppingCartHandler {
  pub fn new(cart_db: Box<dyn CartDb>) -> Self {
    ShoppingCartHandler {
      db: cart_db
    }
  }

  pub fn get_cart(self, id: &str) -> String {
    self.db.get_cart_for(id)
  }
}