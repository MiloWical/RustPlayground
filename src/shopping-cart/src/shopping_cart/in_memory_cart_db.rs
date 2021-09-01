use crate::shopping_cart::cart_db::CartDb;
use std::collections::HashMap;

pub struct InMemoryCartDb {
  db_store: HashMap<String, String>,
}

impl InMemoryCartDb {
  pub fn new() -> Self {
    InMemoryCartDb {
      db_store: HashMap::new(),
    }
  }
}

impl CartDb for InMemoryCartDb {
  fn get_cart_for(&self, cart_id: &str) -> String {
    match self.db_store.get(cart_id) {
      None => "{}".to_string(),
      Some(cart) => cart.to_string(),
    }
  }

  fn add_cart_for(&mut self, cart_id: &str, cart_json: &str) {
    self
      .db_store
      .insert(cart_id.to_string(), cart_json.to_string());
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_ctor() {
    let _db = InMemoryCartDb::new();
  }

  #[test]
  fn test_adding_cart() {
    let mut db = InMemoryCartDb::new();
    db.add_cart_for("a", r#"{"hello": "world"}"#);
  }

  #[test]
  fn test_getting_cart() {
    let mut db = InMemoryCartDb::new();

    let cart_json = r#"{"hello": "world"}"#;
    db.add_cart_for("id", cart_json);
    let cart = db.get_cart_for("id");
    assert_eq!(cart_json, cart);
  }

  #[test]
  fn test_getting_cart_that_doesnt_exist() {
    let db = InMemoryCartDb::new();
    let cart = db.get_cart_for("id2");
    assert_eq!("{}", cart);
  }
}
