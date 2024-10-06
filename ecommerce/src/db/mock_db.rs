use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use uuid::Uuid;
use crate::models::product::{Product, UpdateProduct};
use crate::models::cart::Cart;
use crate::models::checkout::Order;
use crate::models::user::User;

lazy_static! {
    pub static ref DB: Mutex<MockDb> = Mutex::new(MockDb::new());
}

pub struct MockDb {
    products: HashMap<Uuid, Product>,
    carts: HashMap<Uuid, Cart>,
    orders: HashMap<Uuid, Order>,
    users: HashMap<String, User>, // Username as key
}

impl MockDb {
    fn new() -> Self {
        MockDb {
            products: HashMap::new(),
            carts: HashMap::new(),
            orders: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // Product methods
    pub fn add_product(&mut self, product: Product) {
        self.products.insert(product.id, product);
    }

    pub fn get_product(&self, id: &Uuid) -> Option<Product> {
        self.products.get(id).cloned()
    }

    pub fn get_all_products(&self) -> Vec<Product> {
        self.products.values().cloned().collect()
    }

    pub fn update_product(&mut self, id: &Uuid, update: UpdateProduct) -> Option<Product> {
        if let Some(product) = self.products.get_mut(id) {
            if let Some(name) = update.name {
                product.name = name;
            }
            if let Some(description) = update.description {
                product.description = description;
            }
            if let Some(price) = update.price {
                product.price = price;
            }
            if let Some(stock) = update.stock {
                product.stock = stock;
            }
            Some(product.clone())
        } else {
            None
        }
    }

    pub fn delete_product(&mut self, id: &Uuid) -> bool {
        self.products.remove(id).is_some()
    }

    // Cart methods
    pub fn create_cart(&mut self) -> Uuid {
        let cart_id = Uuid::new_v4();
        let cart = Cart::new(cart_id);
        self.carts.insert(cart_id, cart);
        cart_id
    }

    pub fn get_cart(&self, id: &Uuid) -> Option<Cart> {
        self.carts.get(id).cloned()
    }

    pub fn add_to_cart(&mut self, cart_id: &Uuid, product_id: &Uuid, quantity: i32) -> Option<Cart> {
        if let (Some(cart), Some(product)) = (self.carts.get_mut(cart_id), self.products.get(product_id)) {
            cart.add_item(product.clone(), quantity);
            Some(cart.clone())
        } else {
            None
        }
    }

    pub fn remove_from_cart(&mut self, cart_id: &Uuid, product_id: &Uuid) -> Option<Cart> {
        if let Some(cart) = self.carts.get_mut(cart_id) {
            cart.remove_item(product_id);
            Some(cart.clone())
        } else {
            None
        }
    }

    // Order methods
    pub fn create_order(&mut self, cart_id: &Uuid) -> Option<Order> {
        if let Some(cart) = self.carts.remove(cart_id) {
            let order = Order::new(cart);
            let order_id = order.id;
            self.orders.insert(order_id, order.clone());
            Some(order)
        } else {
            None
        }
    }

    pub fn get_order(&self, id: &Uuid) -> Option<Order> {
        self.orders.get(id).cloned()
    }

    // User methods
    pub fn add_user(&mut self, user: User) -> bool {
        if self.users.contains_key(&user.username) {
            false
        } else {
            self.users.insert(user.username.clone(), user);
            true
        }
    }

    pub fn get_user(&self, username: &str) -> Option<User> {
        self.users.get(username).cloned()
    }
}