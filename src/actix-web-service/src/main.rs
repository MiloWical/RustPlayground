// use actix_web::{web, App, HttpServer};
// mod hello_world;
mod shopping_cart;
use shopping_cart::in_memory_cart_db::InMemoryCartDb;
use shopping_cart::cart_db::CartDb;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello_world::hello)
//             .service(hello_world::echo)
//             .route("/hey", web::get().to(hello_world::manual_hello))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

fn main() {
  let db = InMemoryCartDb::new("");
  //println!("{}", InMemoryCartDb::test("<passed param>"));
  let cart_config = db.get_cart("");
  println!("{}", cart_config);
}
