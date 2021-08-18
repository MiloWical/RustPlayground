use actix_web::{web, App, HttpServer};

mod hello_world;
mod shopping_cart;
mod router;

use router::route_config::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// fn main() {
//   let db = InMemoryCartDb::new("");
//   //println!("{}", InMemoryCartDb::test("<passed param>"));
//   let cart_config = db.get_cart("");
//   println!("{}", cart_config);
// }
