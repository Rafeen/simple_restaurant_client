use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewOrderItem {
    pub(crate) item_id: i32,
    pub(crate) quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    pub(crate) id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewItem {
    pub(crate) name: String,
    pub(crate) price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderItem {
    pub(crate) item_id: i32,
    pub(crate) quantity: i32,
    pub(crate) served: bool,
}
