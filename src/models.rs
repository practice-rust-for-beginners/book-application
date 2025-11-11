//! models.rs
//! Contains data structures used to represent Book and related types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a single Book in the library.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Book {
    /// Unique identifier for the book
    pub id: Uuid,
    /// Title of the book
    pub title: String,
    /// Author of the book
    pub author: String,
    /// Optional year of publication
    pub year: Option<u16>,
}

/// Represents an input payload for creating a new book.
/// The server will assign a random UUID.
#[derive(Clone, Debug, Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub year: Option<u16>,
}