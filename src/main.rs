use axum::Router;
pub mod routes;
use routes::order_routes;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new().merge(order_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
// async fn hello_world() {
//     println!("hello_world");
// }
