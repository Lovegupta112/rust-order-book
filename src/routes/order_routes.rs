use crate::{
    input::{CreateOrderInput, DeleteOrderInput, Side},
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
use axum_macros::debug_handler;
use std::sync::{Arc, Mutex, RwLock};
//TODO: TESTING
pub fn order_routes() -> Router {
    //----gloabl level state --->>>
    let global_order_book = Arc::new(Mutex::new(Orderbook::new()));

    Router::new()
        .route("/depth", get(get_depth))
        .with_state(global_order_book.clone())
        .route("/order", post(create_order).delete(delete_order))
        .with_state(global_order_book.clone())
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
#[debug_handler]
async fn create_order(
    State(state): State<Arc<Mutex<Orderbook>>>,
    Json(payload): Json<CreateOrderInput>,
) -> impl IntoResponse {
    // let res = User {
    //     price: payload.price,
    //     quantity: payload.quantity,
    //     user_id: payload.user_id,
    //     side: payload.side,
    // };
    //
    let mut global_orderbook = state.lock().unwrap();

    // match res.side {
    //     Side::Buy => {
    //         global_orderbook.asks.insert(res,)
    //     }
    //     Side::Sell => {}
    // }
    // println!("res: {:?}", res);
    // global_orderbook.add_order();
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
