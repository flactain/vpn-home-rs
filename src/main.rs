use vpn_server_rs::routes::routers;

#[tokio::main]
async fn main() {
    let app = routers();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
