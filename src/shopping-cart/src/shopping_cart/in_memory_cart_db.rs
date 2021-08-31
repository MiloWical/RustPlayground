use std::collections::HashMap;
use crate::shopping_cart::cart_db::CartDb;

pub struct InMemoryCartDb {
  db: Database
}

struct Database {
  connection_string: String,
  db_store: HashMap<String, String>
}

impl InMemoryCartDb {
  pub fn new(conn_str: &str) -> Self {
    InMemoryCartDb {
      db: Database {
        db_store: HashMap::new(),
        connection_string: conn_str.to_string()
      }
    }
  }
}

impl CartDb for InMemoryCartDb
{
  // fn test(x: &str) -> String {
  //   format!("test({}) invoked", x)
  // }

  fn get_cart_for(&self, cart_id: &str) -> String {
    match self.db.db_store.get(cart_id)
    {
      None => "".to_string(),
      Some(cart) => cart.to_string()
    }
  }
}

