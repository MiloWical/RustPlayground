use crate::cart_db::CartDb;

pub struct Router {
  db: CartDb 
}

impl Router {
  pub fn new(database: CartDb) -> Self {
    Router {
      db: database
    }
  }
  
  pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route("/cart/{id}", || get().to(&self.get_cart_for))
  }
}