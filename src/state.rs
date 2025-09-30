use super::input::Side;
use crate::input::CreateOrderInput;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug)]
pub struct Orderbook {
    pub bids: HashMap<u32, Vec<UserOrder>>,
    pub asks: HashMap<u32, Vec<UserOrder>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserOrder {
    user_id: u32,
    qty: u32,
    order_id: String,
}

pub fn order_already_exist(order: UserOrder, order_book: &mut Orderbook, is_bid: bool) {
    let order_type = if is_bid {
        &mut order_book.bids
    } else {
        &mut order_book.asks
    };
    //
    // if order_type.contains_key(&order.price) {
    //     let order_value = order_type.get(&order.price).unwrap();
    //     order_type.insert(order.price, order_value.clone());
    // } else {
    //     let new_order = vec![UserOrder {
    //         user_id: order.user_id,
    //         qty: order.qty,
    //         order_id: order.order_id,
    //     }];
    //     order_type.insert(order.price, new_order);
    // }
}

impl Orderbook {
    pub fn new() -> Self {
        Orderbook {
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, user_order: &CreateOrderInput) {
        let order = vec![UserOrder {
            user_id: user_order.user_id,
            qty: user_order.quantity,
            order_id: Uuid::new_v4().to_string(),
        }];
        match user_order.side {
            Side::Buy => self.bids.insert(user_order.price, order),
            Side::Sell => self.asks.insert(user_order.price, order),
        };
    }

    pub fn show_orderbook(&self) {
        println!("OrderBook: {:?}", self);
    }
}
