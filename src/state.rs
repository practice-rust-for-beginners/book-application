//! state.rs
//! Defines application state and shared data structures.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::Book;

/// The central shared application state.
/// It simulates a database using an in-memory HashMap.
#[derive(Clone, Default)]
pub struct AppState {
    /// A thread-safe, mutable HashMap storing books by their UUID.
    pub books: Arc<Mutex<HashMap<String, Book>>>,
}