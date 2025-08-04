use tokio::net::TcpListener;

mod api;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3010")
        .await
        .expect("can't listen");
    axum::serve(listener, api::make_router()).await.unwrap();
}
