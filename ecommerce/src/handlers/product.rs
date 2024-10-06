use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::product::{Product, CreateProduct, UpdateProduct};
use crate::db::mock_db::DB;

pub async fn get_all_products() -> impl Responder {
    let db = DB.lock().unwrap();
    let products = db.get_all_products();
    HttpResponse::Ok().json(products)
}

pub async fn get_product(path: web::Path<Uuid>) -> impl Responder {
    let product_id = path.into_inner();
    let db = DB.lock().unwrap();

    match db.get_product(&product_id) {
        Some(product) => HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().body("Product not found"),
    }
}

pub async fn create_product(new_product: web::Json<CreateProduct>) -> impl Responder {
    let mut db = DB.lock().unwrap();
    let product = Product {
        id: Uuid::new_v4(),
        name: new_product.name.clone(),
        description: new_product.description.clone(),
        price: new_product.price,
        stock: new_product.stock,
    };

    db.add_product(product.clone());
    HttpResponse::Created().json(product)
}

pub async fn update_product(path: web::Path<Uuid>, update_product: web::Json<UpdateProduct>) -> impl Responder {
    let product_id = path.into_inner();
    let mut db = DB.lock().unwrap();

    match db.update_product(&product_id, update_product.into_inner()) {
        Some(updated_product) => HttpResponse::Ok().json(updated_product),
        None => HttpResponse::NotFound().body("Product not found"),
    }
}

pub async fn delete_product(path: web::Path<Uuid>) -> impl Responder {
    let product_id = path.into_inner();
    let mut db = DB.lock().unwrap();

    if db.delete_product(&product_id) {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("Product not found")
    }
}