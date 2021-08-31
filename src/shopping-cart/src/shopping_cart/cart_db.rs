pub trait CartDb {
  fn get_cart_for(&self, cart_id: &str) -> String;
}