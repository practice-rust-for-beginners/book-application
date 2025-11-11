//! main.rs
//! Entry point for the Book Library REST API.
//!
//! Provides CRUD operations for books using an in-memory HashMap.

mod models;
mod routes;
mod state;

use std::{net::SocketAddr, sync::Arc};
use axum::{
    routing::{get, post, delete},
    Router,
};
use tokio::net::TcpListener;
use state::AppState;
use routes::{get_books, create_book, get_book_by_id, delete_book};

#[tokio::main]
async fn main() {
    // Initialize application state (shared in-memory storage)
    let app_state = Arc::new(AppState::default());

    // Define routes for the REST service
    let app = Router::new()
        .route("/books", get(get_books).post(create_book))
        .route("/books/:id", get(get_book_by_id).delete(delete_book))
        .with_state(app_state);

    // Bind the TCP listener
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Book Library API running at http://{}", addr);

    // Use the new axum::serve function (Axum 0.7+ style)
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}