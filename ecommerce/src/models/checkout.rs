use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::cart::Cart;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: Uuid,
    pub cart: Cart,
    pub total: f64,
    pub status: OrderStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Delivered,
}

impl Order {
    pub fn new(cart: Cart) -> Self {
        Order {
            id: Uuid::new_v4(),
            total: cart.total(),
            status: OrderStatus::Pending,
            cart,
        }
    }
}