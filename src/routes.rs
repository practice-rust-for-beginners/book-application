//! routes.rs
//! Defines all REST API endpoints for the Book Library.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use std::sync::Arc;

use crate::{models::{Book, CreateBook}, state::AppState};

/// Handler to retrieve all books in the library.
pub async fn get_books(State(state): State<Arc<AppState>>) -> Json<Vec<Book>> {
    let books_guard = state.books.lock().unwrap();
    let books_list: Vec<Book> = books_guard.values().cloned().collect();
    Json(books_list)
}

/// Handler to create a new book entry.
pub async fn create_book(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateBook>,
) -> (StatusCode, Json<Book>) {
    let mut books_guard = state.books.lock().unwrap();
    let new_book = Book {
        id: Uuid::new_v4(),
        title: payload.title,
        author: payload.author,
        year: payload.year,
    };
    books_guard.insert(new_book.id.to_string(), new_book.clone());
    (StatusCode::CREATED, Json(new_book))
}

/// Handler to retrieve a single book by ID.
pub async fn get_book_by_id(
    State(state): State<Arc<AppState>>,
    Path(book_id): Path<String>,
) -> Result<Json<Book>, StatusCode> {
    let books_guard = state.books.lock().unwrap();
    if let Some(book) = books_guard.get(&book_id) {
        Ok(Json(book.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Handler to delete a book by ID.
pub async fn delete_book(
    State(state): State<Arc<AppState>>,
    Path(book_id): Path<String>,
) -> StatusCode {
    let mut books_guard = state.books.lock().unwrap();
    if books_guard.remove(&book_id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}