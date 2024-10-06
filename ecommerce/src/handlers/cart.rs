use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::db::mock_db::DB;

pub async fn create_cart() -> impl Responder {
    let mut db = DB.lock().unwrap();
    let cart_id = db.create_cart();
    HttpResponse::Created().json(json!({"cart_id": cart_id}))
}

pub async fn get_cart(path: web::Path<Uuid>) -> impl Responder {
    let cart_id = path.into_inner();
    let db = DB.lock().unwrap();

    match db.get_cart(&cart_id) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound().body("Cart not found"),
    }
}

#[derive(Deserialize)]
pub struct AddToCartRequest {
    product_id: Uuid,
    quantity: i32,
}

pub async fn add_to_cart(path: web::Path<Uuid>, req: web::Json<AddToCartRequest>) -> impl Responder {
    let cart_id = path.into_inner();
    let mut db = DB.lock().unwrap();

    match db.add_to_cart(&cart_id, &req.product_id, req.quantity) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound().body("Cart or Product not found"),
    }
}

pub async fn remove_from_cart(path: web::Path<(Uuid, Uuid)>) -> impl Responder {
    let (cart_id, product_id) = path.into_inner();
    let mut db = DB.lock().unwrap();

    match db.remove_from_cart(&cart_id, &product_id) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound().body("Cart not found"),
    }
}