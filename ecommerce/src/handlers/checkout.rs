use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;
use crate::db::mock_db::DB;

#[derive(Deserialize)]
pub struct CheckoutRequest {
    cart_id: Uuid,
}

pub async fn process_checkout(req: web::Json<CheckoutRequest>) -> impl Responder {
    let mut db = DB.lock().unwrap();

    match db.create_order(&req.cart_id) {
        Some(order) => HttpResponse::Created().json(order),
        None => HttpResponse::NotFound().body("Cart not found"),
    }
}

pub async fn get_order(path: web::Path<Uuid>) -> impl Responder {
    let order_id = path.into_inner();
    let db = DB.lock().unwrap();

    match db.get_order(&order_id) {
        Some(order) => HttpResponse::Ok().json(order),
        None => HttpResponse::NotFound().body("Order not found"),
    }
}