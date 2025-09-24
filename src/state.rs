use super::input::Side;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Orderbook {
    pub bids: HashMap<u32, Vec<UserOrder>>,
    pub asks: HashMap<u32, Vec<UserOrder>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserOrder {
    user_id: String,
    qty: u32,
    order_id: u32,
}

pub fn order_already_exist(order: UserOrder, order_book: &mut Orderbook, is_bid: bool) {
    let order_type = if is_bid {
        &mut order_book.bids
    } else {
        &mut order_book.asks
    };

    if order_type.contains_key(&order.order_id) {
        let order_value = order_type.get(&order.order_id).unwrap();
        order_type.insert(order.order_id, order_value.clone());
    } else {
        let new_order = vec![UserOrder {
            user_id: order.user_id,
            qty: order.qty,
            order_id: order.order_id,
        }];
        order_type.insert(order.order_id, new_order);
    }
}

impl Orderbook {
    pub fn new() -> Self {
        Orderbook {
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, user_order: UserOrder, side: Side) {
        let order = vec![UserOrder {
            user_id: user_order.user_id,
            qty: user_order.qty,
            order_id: user_order.order_id,
        }];
        match side {
            Side::Buy => self.asks.insert(user_order.order_id, order),
            Side::Sell => self.bids.insert(user_order.order_id, order),
        };
    }

    pub fn show_orderbook(&self) {
        println!("OrderBook: {:?}", self);
    }
}
