use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderResponse {
    pub order_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteOrderResponse {
    pub filled_qty: u32,
    pub avg_price: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthResponse {
    pub bids: Vec<[u32; 2]>,
    pub asks: Vec<[u32; 2]>,
    pub last_update_id: String,
}
