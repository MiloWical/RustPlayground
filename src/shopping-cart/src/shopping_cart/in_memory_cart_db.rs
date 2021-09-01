use std::collections::HashMap;
use crate::shopping_cart::cart_db::CartDb;

pub struct InMemoryCartDb {
  db_store: HashMap<String, String>
}

impl InMemoryCartDb {
  pub fn new() -> Self {
    InMemoryCartDb {
      db_store: HashMap::new()
    }
  }
}

impl CartDb for InMemoryCartDb
{
  fn get_cart_for(&self, cart_id: &str) -> String {
    match self.db_store.get(cart_id)
    {
      None => "".to_string(),
      Some(cart) => cart.to_string()
    }
  }

  fn add_cart_for(&mut self, cart_id: &str, cart_json: &str) {
    self.db_store.insert(cart_id.to_string(), cart_json.to_string());
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
    fn test_ctor() {
        let db = InMemoryCartDb::new();
    }
}