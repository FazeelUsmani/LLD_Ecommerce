use actix_web::{web, HttpResponse, Responder};
use crate::models::user::{User, LoginCredentials};
use crate::db::mock_db::DB;

pub async fn register(user: web::Json<User>) -> impl Responder {
    let mut db = DB.lock().unwrap();

    if db.add_user(user.into_inner()) {
        HttpResponse::Created().body("User registered successfully")
    } else {
        HttpResponse::BadRequest().body("Username already exists")
    }
}

pub async fn login(creds: web::Json<LoginCredentials>) -> impl Responder {
    let db = DB.lock().unwrap();

    match db.get_user(&creds.username) {
        Some(user) if user.password_hash == creds.password => { // Note: In real app, use proper password verification
            HttpResponse::Ok().body("Login successful")
        },
        _ => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}