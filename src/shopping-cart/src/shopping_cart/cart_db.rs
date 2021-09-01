pub trait CartDb {
  fn get_cart_for(&self, cart_id: &str) -> String;
  fn add_cart_for(&mut self, cart_id: &str, cart_json: &str);
}