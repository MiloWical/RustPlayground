use std::collections::HashMap;
use crate::shopping_cart::cart_db::CartDb;

pub struct InMemoryCartDb {
  db: Database
}

struct Database {
  connection_string: String,
  db_store: HashMap<String, String>
}

impl CartDb for InMemoryCartDb
{
  // fn test(x: &str) -> String {
  //   format!("test({}) invoked", x)
  // }

  fn new(conn_str: &str) -> Self {
    InMemoryCartDb {
      db: Database {
        db_store: HashMap::new(),
        connection_string: conn_str.to_string()
      }
    }
  }

  fn get_cart(&self, cart_id: &str) -> String {
    "Got the cart!".to_string()
  }
}