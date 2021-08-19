use actix_web::{web, App, HttpServer};

mod hello_world;
mod shopping_cart;
mod router;
mod controllers;
mod handlers;

use crate::router::route_config::routes;
use crate::shopping_cart::in_memory_cart_db::InMemoryCartDb;
use crate::handlers::shopping_cart_handler::ShoppingCartHandler;
use crate::controllers::controller::Controller;
use crate::controllers::shopping_cart_controller::ShoppingCartController;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let controller = ShoppingCartController::new(
      
    // );

    HttpServer::new(|| {
        App::new()
            .configure(routes)
            .configure(<ShoppingCartController as Controller>::configure_routes)
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
