use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::models::product::Product;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartItem {
    pub product: Product,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cart {
    pub id: Uuid,
    pub items: HashMap<Uuid, CartItem>,
}

impl Cart {
    pub fn new(id: Uuid) -> Self {
        Cart {
            id,
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, product: Product, quantity: i32) {
        self.items.entry(product.id)
            .and_modify(|item| item.quantity += quantity)
            .or_insert(CartItem { product, quantity });
    }

    pub fn remove_item(&mut self, product_id: &Uuid) {
        self.items.remove(product_id);
    }

    pub fn total(&self) -> f64 {
        self.items.values().fold(0.0, |acc, item| {
            acc + item.product.price * item.quantity as f64
        })
    }
}