mod database;
mod models {
    pub mod note;
}
mod handlers {
    pub mod note_handler;
    pub mod ui_handler;
}
mod router;

use std::net::SocketAddr;
use router::create_router;
use database::init_db;

#[tokio::main]
async fn main() {
    let pool = init_db().await;

    let app = create_router(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
