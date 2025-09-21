use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

pub fn order_routes() -> Router {
    Router::new()
        .route("/depth", get(get_depth))
        .route("/order", post(create_order).delete(delete_order))
}

#[derive(Serialize, Deserialize)]
struct CreateOrderInput {
    price: u32,
    quantity: u32,
    user_id: u32,
    side: Side,
}

#[derive(Serialize, Deserialize)]
enum Side {
    Buy,
    Sell,
}

async fn get_depth() -> impl IntoResponse {
    "Hello world"
}

async fn create_order(Json(payload): Json<CreateOrderInput>) -> impl IntoResponse {
    let res = CreateOrderInput {
        price: payload.price,
        quantity: payload.quantity,
        user_id: payload.user_id,
        side: payload.side,
    };

    (StatusCode::CREATED, Json(res))
}

async fn delete_order() -> impl IntoResponse {
    "Hello world"
}

