use crate::shopping_cart::cart_db::CartDb;

pub struct ShoppingCart<T: CartDb> {
  database: T
}

impl<T> ShoppingCart<T>
  where
    T: CartDb
{
  pub fn new(db: T) -> Self
  {
    ShoppingCart {
      database: db
    }
  }
}