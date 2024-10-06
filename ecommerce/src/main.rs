use actix_web::{web, App, HttpServer};
use std::io;

mod models;
mod handlers;
mod db;

#[cfg(test)]
mod tests;

use crate::handlers::product::{get_all_products, create_product, get_product, update_product, delete_product};
use crate::handlers::cart::{create_cart, get_cart, add_to_cart, remove_from_cart};
use crate::handlers::checkout::{process_checkout, get_order};
use crate::handlers::user::{register, login};

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/products")
                    .route("", web::get().to(get_all_products))
                    .route("", web::post().to(create_product))
                    .route("/{id}", web::get().to(get_product))
                    .route("/{id}", web::put().to(update_product))
                    .route("/{id}", web::delete().to(delete_product))
            )
            .service(
                web::scope("/cart")
                    .route("", web::post().to(create_cart))
                    .route("/{id}", web::get().to(get_cart))
                    .route("/{id}/add", web::post().to(add_to_cart))
                    .route("/{id}/remove/{product_id}", web::delete().to(remove_from_cart))
            )
            .service(
                web::scope("/checkout")
                    .route("", web::post().to(process_checkout))
                    .route("/order/{id}", web::get().to(get_order))
            )
            .service(
                web::scope("/users")
                    .route("/register", web::post().to(register))
                    .route("/login", web::post().to(login))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}