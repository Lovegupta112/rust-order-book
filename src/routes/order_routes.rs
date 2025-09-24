use crate::{
    input::{CreateOrderInput, DeleteOrderInput},
    output::{CreateOrderResponse, DeleteOrderResponse, DepthResponse},
    state::Orderbook,
};
use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use std::sync::{Arc, Mutex, RwLock};

pub fn order_routes() -> Router {
    let global_order_book = Arc::new(Mutex::new(Orderbook::new()));
    Router::new()
        .route("/depth", get(get_depth))
        .with_state(global_order_book)
        .route("/order", post(create_order).delete(delete_order))
}

async fn get_depth(State(state): State<Arc<Mutex<Orderbook>>>) -> impl IntoResponse {
    let bids = "hello";
    (
        StatusCode::OK,
        Json(DepthResponse {
            bids: vec![],
            asks: vec![],
            last_update_id: "itzltg".to_string(),
        }),
    )
}

async fn create_order(Json(payload): Json<CreateOrderInput>) -> impl IntoResponse {
    let res = CreateOrderInput {
        price: payload.price,
        quantity: payload.quantity,
        user_id: payload.user_id,
        side: payload.side,
    };

    println!("res: {:?}", res);

    (
        StatusCode::CREATED,
        Json(CreateOrderResponse {
            order_id: "itzltg".to_string(),
        }),
    )
}

async fn delete_order(Json(payload): Json<DeleteOrderInput>) -> impl IntoResponse {
    println!("Deleted order:{:?}", payload.order_id);
    (
        StatusCode::OK,
        // format!("order {} is deleted successfully.", payload.order_id),
        Json(DeleteOrderResponse {
            filled_qty: 0,
            avg_price: 200,
        }),
    )
}
