
# 📚 Book Library - REST API in Rust

A simple, well-documented **RESTful API** built using the [Axum](https://crates.io/crates/axum) framework in Rust.  
This project demonstrates how to create a lightweight in-memory REST service to manage a collection of books.

---

## 🚀 Overview

The **Book Library API** allows you to:

- Add (`POST`) new books
- Retrieve (`GET`) all books
- Retrieve (`GET`) a specific book by ID
- Delete (`DELETE`) a book by ID

It uses a **HashMap** (wrapped with `Arc<Mutex<>>`) to simulate a database, holding data in memory while the application runs.

---

## 🧩 Project Structure

```
book_library/
├── Cargo.toml            # Project configuration and dependencies
└── src/
    ├── main.rs           # Application entry point — starts the Axum web server
    ├── models.rs         # Defines Book structs and request models
    ├── routes.rs         # HTTP route handlers for CRUD operations
    └── state.rs          # Shared application state (in-memory storage)
```

---

## 🧠 Module-Level Description

### **1. main.rs**
- Initializes the web server.
- Registers routes and application state.
- Starts the Axum HTTP server at `http://127.0.0.1:3000`.

### **2. models.rs**
- Defines the **Book** and **CreateBook** data structures.
- Uses [serde](https://crates.io/crates/serde) for JSON serialization/deserialization.
- Provides structure for incoming/outgoing data.

### **3. routes.rs**
- Contains handlers for REST endpoints:
  - `get_books` → Returns a list of all books.
  - `create_book` → Adds a new book.
  - `get_book_by_id` → Fetches details of a specific book.
  - `delete_book` → Deletes a book by ID.
- Returns appropriate HTTP status codes (`200`, `201`, `204`, `404`).

### **4. state.rs**
- Defines the global application state type `AppState`.
- Stores book records in a **thread-safe** `HashMap<String, Book>` using `Arc<Mutex<_>>`.
- Enables sharing the same data across all API routes.

---

## ⚙️ Dependencies

Defined in `Cargo.toml`:

- **axum** — Web framework for building async REST APIs.
- **tokio** — Asynchronous runtime used by Axum.
- **serde / serde_json** — Serialization/deserialization for JSON handling.
- **uuid** — To generate unique identifiers for books.

---

## 🧰 Setup & Running

### 1️⃣ Clone and Build
```bash
git clone https://github.com/yourusername/book_library.git
cd book_library
cargo build
```

### 2️⃣ Run the Server
```bash
cargo run
```

Output:
```
🚀 Book Library API running at http://127.0.0.1:3000
```

The service will start on **http://127.0.0.1:3000**

---

## 🧪 Testing the API

Use `curl` or an HTTP client like Postman.

### ➕ **Add a new book**
```bash
curl -X POST http://127.0.0.1:3000/books \
  -H "Content-Type: application/json" \
  -d '{"title":"The Alchemist","author":"Paulo Coelho","year":1988}'
```

**Response:**
```json
{
  "id": "4ac7b680-9e4a-4b37-a003-57d4b328f3ea",
  "title": "The Alchemist",
  "author": "Paulo Coelho",
  "year": 1988
}
```

---

### 📖 **Get all books**
```bash
curl http://127.0.0.1:3000/books
```

---

### 🔍 **Get a book by ID**
```bash
curl http://127.0.0.1:3000/books/<uuid>
```

Example:
```bash
curl http://127.0.0.1:3000/books/4ac7b680-9e4a-4b37-a003-57d4b328f3ea
```

---

### ❌ **Delete a book**
```bash
curl -X DELETE http://127.0.0.1:3000/books/<uuid>
```

---

## 🧾 API Routes Summary

| HTTP Method | Endpoint           | Description            | Request Body                  | Response |
|--------------|-------------------|------------------------|--------------------------------|-----------|
| `GET`        | `/books`          | Get all books          | None                           | `200 OK`  |
| `POST`       | `/books`          | Create a new book      | `{ "title", "author", "year"}` | `201 Created` |
| `GET`        | `/books/:id`      | Get book by ID         | None                           | `200 OK` / `404` |
| `DELETE`     | `/books/:id`      | Delete book by ID      | None                           | `204 No Content` / `404` |

---

## 🧼 Notes

- Data is **in-memory only** — once the app stops, all data is lost.
- You can extend this project later by:
  - Adding file or database persistence (SQLite, PostgreSQL, etc.).
  - Implementing update (`PUT`) operations.
  - Using structured logging or error handling middleware.

---

## 📜 License

This project is licensed under the **MIT License**.

---

## 👨‍💻 Author [Aditya Pratap Bhuyan](https://www.linkedin.com/in/adityabhuyan/)

*Developed with ❤️ in Rust for learning and demonstration purposes.*

---
