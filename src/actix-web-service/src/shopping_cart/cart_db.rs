pub trait CartDb {
  //fn new(conn_str: &str) -> Self;

  //fn test(x: &str) -> String;

  fn get_cart_for(&self, cart_id: &str) -> String;
}